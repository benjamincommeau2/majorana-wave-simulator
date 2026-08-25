/*
Purpose: build GPU command buffers for compute operations.

This module will keep command-encoder and compute-pass boilerplate out of `lib.rs`.

The first responsibility moved here will be recording the GPU J operation.
*/

pub fn create_command_encoder( // Defines a helper that creates the command encoder used to record our GPU operations.

  device: &wgpu::Device, // Borrows the existing WebGPU device that is responsible for creating command encoders.

) -> wgpu::CommandEncoder { // Returns the newly created command encoder to the caller.

  device.create_command_encoder( // Asks the WebGPU device to create a command encoder.

    &wgpu::CommandEncoderDescriptor { // Starts the descriptor that configures the command encoder.

      label: Some("Majorana Readback Encoder"), // Preserves the same debugging label currently used in `lib.rs`.

    }, // Finishes the command-encoder descriptor.

  ) // Returns the created command encoder from this helper function.

} // Closes the command-encoder creation helper.

pub fn record_apply_j( // Defines a helper that records the GPU commands needed to apply J to the bound Majorana state.

  encoder: &mut wgpu::CommandEncoder, // Borrows the existing command encoder mutably so this function can add compute commands to it.

  pipeline: &wgpu::ComputePipeline, // Borrows the existing Apply J compute pipeline that contains our WGSL shader.

  bind_group: &wgpu::BindGroup, // Borrows the bind group that connects the Majorana state buffer to the shader.

) { // Starts the command-recording helper.

  let mut compute_pass = encoder.begin_compute_pass( // Begins recording a compute pass into the existing command encoder.

    &wgpu::ComputePassDescriptor { // Starts the descriptor that configures this compute pass.

      label: Some("Apply J Compute Pass"), // Preserves the existing readable debugging label for the compute pass.

      timestamp_writes: None, // Keeps GPU timestamp measurements disabled because this checkpoint is about correctness rather than profiling.

    }, // Finishes the compute-pass descriptor.

  ); // Finishes creating the compute pass.

  compute_pass.set_pipeline(pipeline); // Selects the Apply J compute pipeline for this compute pass.

  compute_pass.set_bind_group( // Connects the GPU resources required by the shader.

    0, // Selects bind group zero to match `@group(0)` in the WGSL shader.

    bind_group, // Supplies the bind group containing the Majorana state buffer.

    &[], // Supplies no dynamic offsets because our current buffer binding does not use them.

  ); // Finishes setting bind group zero.

  compute_pass.dispatch_workgroups( // Records the command that launches the Apply J compute shader.

    1, // Dispatches one workgroup in the x direction.

    1, // Dispatches one workgroup in the y direction.

    1, // Dispatches one workgroup in the z direction.

  ); // Finishes recording the compute dispatch.

} // Ends the helper and also ends the compute pass when `compute_pass` goes out of scope.

pub fn record_readback_copy( // Defines a helper that records the copy from the GPU state buffer into the CPU-readable staging buffer.

  encoder: &mut wgpu::CommandEncoder, // Borrows the existing command encoder mutably so this function can add the copy command.

  state_buffer: &wgpu::Buffer, // Borrows the GPU buffer containing the current Majorana state.

  readback_buffer: &wgpu::Buffer, // Borrows the CPU-readable staging buffer that will receive the copied state bytes.

  size: wgpu::BufferAddress, // Receives the exact number of bytes that should be copied between the two buffers.

) { // Starts the readback-copy helper.

  encoder.copy_buffer_to_buffer( // Records a GPU buffer-to-buffer copy command into the existing encoder.

    state_buffer, // Uses the Majorana state buffer as the source of the copy.

    0, // Starts reading from byte offset zero in the source buffer.

    readback_buffer, // Uses the CPU-readable staging buffer as the destination of the copy.

    0, // Starts writing from byte offset zero in the destination buffer.

    size, // Copies exactly the number of bytes supplied by the caller.

  ); // Finishes recording the buffer-copy command.

} // Closes the readback-copy helper.

pub fn submit_commands( // Defines a helper that finishes a recorded command encoder and submits its commands to the GPU queue.

  queue: &wgpu::Queue, // Borrows the existing GPU queue that is responsible for submitting work to WebGPU.

  encoder: wgpu::CommandEncoder, // Takes ownership of the completed command encoder because finishing it consumes the encoder.

) { // Starts the command-submission helper.

  queue.submit( // Submits completed GPU commands to the queue for execution.

    Some( // Wraps the completed command buffer because `submit` accepts an iterator of command buffers.

      encoder.finish(), // Finishes recording and converts the command encoder into an executable command buffer.

    ), // Finishes wrapping the command buffer in `Some`.

  ); // Finishes submitting the command buffer to the GPU queue.

} // Closes the command-submission helper.