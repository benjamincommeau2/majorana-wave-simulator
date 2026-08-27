// tests/gpu_apply_j_integration_tests.rs

use majorana_wave_simulator::gpu; // Gives this integration test access to the same production WebGPU helpers used by the browser application.

use majorana_wave_simulator::physics; // Gives the test access to the independently tested CPU J reference implementation.

use majorana_wave_simulator::state::MajoranaState; // Gives the test access to the production four-component Majorana state representation.

#[test] // Verifies the real native wgpu compute path for the Apply J shader.
#[ignore = "requires a native GPU adapter"] // Keeps ordinary cargo test and hardware-agnostic CI from failing when no GPU adapter is available.
fn gpu_apply_j_matches_cpu_reference() {

  pollster::block_on(async { // Lets this ordinary Rust test wait for wgpu's asynchronous adapter and device creation.

    let instance = gpu::gpu_context::create_instance(); // Creates a real wgpu instance using the same production helper as the application.

    let adapter = gpu::gpu_context::request_adapter(&instance).await; // Requests a real native GPU adapter through the production GPU-context code.

    let (device, queue) = gpu::gpu_context::request_device_and_queue(&adapter).await; // Creates the real GPU device and queue used by the integration test.

    let apply_j_shader = gpu::shader_modules::create_apply_j_shader(&device); // Compiles the actual production apply_j.wgsl shader.

    let apply_j_pipeline = gpu::pipelines::create_apply_j_pipeline( // Creates the actual production compute pipeline.

      &device, // Supplies the real GPU device.

      &apply_j_shader, // Supplies the compiled production Apply J shader.

    ); // Finishes creating the compute pipeline.

    let majorana_state = MajoranaState::new(); // Creates the same tested initial Majorana state used by the application.

    let majorana_components = majorana_state.components(); // Borrows the four f32 components that will be uploaded to GPU memory.

    let buffer_size = std::mem::size_of_val(majorana_components) as wgpu::BufferAddress; // Calculates the exact sixteen-byte size of the Majorana state.

    let (state_buffer, readback_buffer) = gpu::state_buffers::create_majorana_buffers( // Creates the actual production GPU state and readback buffers.

      &device, // Supplies the real GPU device.

      buffer_size, // Allocates exactly enough memory for one four-component state.

    ); // Finishes creating the GPU buffers.

    let apply_j_bind_group = gpu::bind_groups::create_apply_j_bind_group( // Connects the state buffer to the production Apply J pipeline.

      &device, // Supplies the real GPU device.

      &apply_j_pipeline, // Supplies the real compute pipeline whose binding layout must be satisfied.

      &state_buffer, // Supplies the state buffer that the shader will transform.

    ); // Finishes creating the production bind group.

    let mut encoder = gpu::commands::create_command_encoder(&device); // Creates a real command encoder through the production command helper.

    gpu::commands::record_apply_j( // Records the actual Apply J compute dispatch.

      &mut encoder, // Supplies the command encoder that records this GPU operation.

      &apply_j_pipeline, // Supplies the production Apply J compute pipeline.

      &apply_j_bind_group, // Supplies the production bind group connected to the state buffer.

    ); // Finishes recording the compute dispatch.

    gpu::commands::record_readback_copy( // Records the copy from transformed GPU state into CPU-readable staging memory.

      &mut encoder, // Uses the same command encoder after the Apply J dispatch.

      &state_buffer, // Copies from the GPU state that the shader transformed.

      &readback_buffer, // Copies into the CPU-readable production readback buffer.

      buffer_size, // Copies exactly sixteen bytes.

    ); // Finishes recording the readback copy.

    queue.write_buffer( // Uploads the original Majorana state before submitting the recorded GPU commands.

      &state_buffer, // Selects the real GPU state buffer.

      0, // Starts writing at the beginning of the buffer.

      bytemuck::cast_slice(majorana_components), // Converts the four f32 components into their sixteen-byte GPU representation.

    ); // Finishes uploading the input state.

    gpu::commands::submit_commands( // Submits the real Apply J dispatch and readback copy to the GPU.

      &queue, // Supplies the real GPU command queue.

      encoder, // Transfers the completed command encoder for submission.

    ); // Finishes submitting the GPU work.

    let readback_slice = readback_buffer.slice(..); // Selects the complete sixteen-byte readback buffer for CPU mapping.

    let (sender, receiver) = std::sync::mpsc::channel(); // Creates a small channel so the asynchronous mapping callback can report completion back to the test.

    readback_slice.map_async( // Requests CPU-readable access after the submitted GPU work completes.

      wgpu::MapMode::Read, // Requests read-only CPU mapping.

      move |map_result| { // Runs after wgpu finishes preparing the readback buffer.

        sender.send(map_result).expect("Could not send GPU map result to integration test"); // Sends the mapping result back to the waiting test thread.

      }, // Finishes the mapping callback.

    ); // Finishes requesting readback-buffer mapping.

    device.poll( // Explicitly drives native wgpu until the submitted GPU work and mapping callback complete.

      wgpu::PollType::wait_indefinitely(), // Waits for the most recent GPU submission without imposing an arbitrary timeout.

    ).expect("Could not wait for GPU integration-test work to complete"); // Stops clearly if native wgpu cannot complete the submitted work.

    receiver // Waits for the mapping callback to report whether CPU access succeeded.

      .recv() // Receives the result sent by the map_async callback.

      .expect("GPU mapping callback did not report a result") // Stops clearly if the callback channel closes unexpectedly.

      .expect("GPU readback buffer could not be mapped"); // Stops clearly if wgpu itself reports a mapping failure.

    let mapped_range = readback_buffer // Begins reading the bytes produced by the actual GPU shader.

      .slice(..) // Selects the complete sixteen-byte readback buffer.

      .get_mapped_range() // Obtains the CPU-visible mapped bytes.

      .expect("Could not access mapped GPU integration-test bytes"); // Stops clearly if mapped bytes cannot be accessed.

    let gpu_state = MajoranaState::from_bytes(&mapped_range); // Reconstructs the real GPU result through the production state API.

    let cpu_expected = physics::j::apply_j(majorana_components); // Computes the independently tested CPU reference result.

    assert_eq!( // Requires the actual WGSL computation to agree exactly with the CPU J reference.

      gpu_state.components(), // Supplies the four components returned from the real GPU shader.

      &cpu_expected, // Supplies the four components calculated by the CPU reference implementation.

    ); // Finishes comparing GPU and CPU results.

    drop(mapped_range); // Releases the mapped byte view before unmapping the underlying GPU buffer.

    readback_buffer.unmap(); // Returns the readback buffer to the unmapped state after the test has finished reading it.

  }); // Finishes the asynchronous native GPU integration-test body.

} // Finishes the Apply J GPU integration test.