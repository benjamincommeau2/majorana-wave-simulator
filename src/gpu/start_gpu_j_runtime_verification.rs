// src/gpu/start_gpu_j_runtime_verification.rs

use crate::state; // Gives the runtime-verification procedure access to the tested Majorana state representation.

use super::bind_groups; // Gives the procedure access to the existing Apply J bind-group helper.

use super::commands; // Gives the procedure access to the existing GPU command-recording and submission helpers.

use super::gpu_j_readback_verification; // Gives the runtime procedure access to the independently tested GPU J result checker.

use super::pipelines; // Gives the procedure access to the existing Apply J compute-pipeline helper.

use super::shader_modules; // Gives the procedure access to the existing Apply J WGSL shader-module helper.

use super::state_buffers; // Gives the procedure access to the existing Majorana GPU-buffer creation helper.

pub(crate) fn start_gpu_j_runtime_verification( // Starts the GPU J dispatch, readback, and asynchronous comparison against the CPU reference.

  device: &wgpu::Device, // Borrows the existing WebGPU device used to create the compute resources.

  queue: &wgpu::Queue, // Borrows the existing WebGPU queue used to upload the state and submit the commands.

  status: &web_sys::Element, // Borrows the browser status element so runtime verification progress and results remain visible.

) { // Starts the complete GPU J runtime-verification procedure.

  let apply_j_shader = shader_modules::create_apply_j_shader(device); // Creates the WGSL shader module that implements the Majorana J operation.

  let apply_j_pipeline = pipelines::create_apply_j_pipeline( // Creates the compute pipeline used to execute the Apply J shader.

    device, // Supplies the existing WebGPU device.

    &apply_j_shader, // Supplies the compiled Apply J shader module.

  ); // Finishes creating the Apply J compute pipeline.

  let majorana_state = state::MajoranaState::new(); // Creates the initial four-component Majorana state through the tested production API.

  let majorana_components = majorana_state.components(); // Borrows the four real components that will be uploaded to GPU memory.

  let buffer_size = std::mem::size_of_val(majorana_components) as wgpu::BufferAddress; // Calculates the exact sixteen-byte storage size of one Majorana state.

  let (state_buffer, readback_buffer) = state_buffers::create_majorana_buffers( // Creates the writable GPU state buffer and CPU-readable staging buffer.

    device, // Supplies the existing WebGPU device.

    buffer_size, // Allocates exactly enough storage for the four-component Majorana state.

  ); // Finishes creating the state and readback buffers.

  let apply_j_bind_group = bind_groups::create_apply_j_bind_group( // Connects the Majorana state buffer to the Apply J compute pipeline.

    device, // Supplies the existing WebGPU device.

    &apply_j_pipeline, // Supplies the pipeline whose inferred binding layout must be satisfied.

    &state_buffer, // Supplies the GPU buffer that the shader will transform in place.

  ); // Finishes creating the Apply J bind group.

  let mut encoder = commands::create_command_encoder(device); // Creates the command encoder used to record the J dispatch and readback copy.

  commands::record_apply_j( // Records the compute pass that applies J to the GPU-resident Majorana state.

    &mut encoder, // Supplies mutable access to the command encoder.

    &apply_j_pipeline, // Supplies the Apply J compute pipeline.

    &apply_j_bind_group, // Supplies the bind group connected to the state buffer.

  ); // Finishes recording the Apply J compute pass.

  commands::record_readback_copy( // Records the copy from the transformed state buffer into the CPU-readable staging buffer.

    &mut encoder, // Supplies mutable access to the same command encoder.

    &state_buffer, // Supplies the transformed GPU state as the copy source.

    &readback_buffer, // Supplies the CPU-readable staging buffer as the copy destination.

    buffer_size, // Copies exactly one sixteen-byte Majorana state.

  ); // Finishes recording the readback copy.

  queue.write_buffer( // Uploads the original Majorana state before the recorded compute commands execute.

    &state_buffer, // Selects the GPU state buffer that the Apply J shader will transform.

    0, // Starts writing at the beginning of the state buffer.

    bytemuck::cast_slice(majorana_components), // Reinterprets the four f32 components as their sixteen-byte GPU representation.

  ); // Finishes uploading the original Majorana state.

  commands::submit_commands( // Finishes the encoder and submits the complete J-dispatch and readback-copy command buffer.

    queue, // Supplies the existing WebGPU command queue.

    encoder, // Transfers ownership of the completed command encoder.

  ); // Finishes submitting the GPU commands.

  let readback_slice = readback_buffer.slice(..); // Creates a view covering all sixteen bytes that will be mapped back to CPU-visible memory.

  let readback_buffer_for_callback = readback_buffer.clone(); // Keeps a buffer handle alive for the asynchronous mapping callback.

  let uploaded_components_for_verification = *majorana_components; // Copies the original CPU state so the asynchronous callback can calculate the expected J result later.

  let status_for_readback = status.clone(); // Keeps the browser status element available to the asynchronous mapping callback.

  readback_slice.map_async( // Requests asynchronous read-only access to the completed GPU readback buffer.

    wgpu::MapMode::Read, // Requests CPU-readable mapping without allowing writes.

    move |map_result| { // Starts the callback that handles success or failure of GPU memory mapping.

      match map_result { // Examines whether WebGPU successfully mapped the readback buffer.

        Ok(()) => { // Handles successful access to the GPU-returned bytes.

          let mapped_range = readback_buffer_for_callback // Begins obtaining the CPU-visible mapped bytes.

            .slice(..) // Selects the complete readback buffer.

            .get_mapped_range() // Requests access to the mapped byte range.

            .expect("Could not access mapped readback bytes"); // Stops clearly if mapped bytes cannot be obtained after successful mapping.

          if gpu_j_readback_verification::gpu_j_readback_matches_cpu_reference( // Delegates scientific correctness checking to the independently tested pure verifier.

            &mapped_range, // Supplies the sixteen bytes returned from the GPU.

            &uploaded_components_for_verification, // Supplies the original CPU state used to calculate the expected J transformation.

          ) { // Handles a GPU result that exactly matches the CPU J reference.

            status_for_readback.set_text_content( // Updates the webpage with the successful runtime-verification result.

              Some("GPU J operation verified against CPU reference."), // States precisely what the browser runtime has verified.

            ); // Finishes reporting successful verification.

          } else { // Handles a mapped GPU result that disagrees with the CPU J reference.

            status_for_readback.set_text_content( // Updates the webpage with a scientifically specific verification failure.

              Some("GPU J verification failed: readback values did not match the CPU J reference."), // States that the disagreement is between GPU J output and the tested CPU reference.

            ); // Finishes reporting the verification failure.

          } // Finishes choosing between matching and mismatching GPU results.

          drop(mapped_range); // Releases the CPU-visible mapped byte view before unmapping the underlying GPU buffer.

          readback_buffer_for_callback.unmap(); // Releases the buffer mapping after verification is complete.

        } // Finishes handling successful GPU buffer mapping.

        Err(map_error) => { // Handles failure to make the GPU readback buffer accessible to CPU code.

          status_for_readback.set_text_content( // Makes the WebGPU mapping failure visible in the browser.

            Some(&format!("GPU readback mapping failed: {map_error:?}")), // Includes the WebGPU mapping error in the visible status message.

          ); // Finishes displaying the mapping failure.

        } // Finishes handling unsuccessful GPU buffer mapping.

      } // Finishes examining the asynchronous mapping result.

    }, // Finishes the GPU readback callback.

  ); // Finishes requesting CPU-readable mapping of the GPU result.

  status.set_text_content( // Preserves the existing immediate status update while the asynchronous mapping callback is pending.

    Some("Majorana state uploaded to GPU successfully."), // Reports that the initial CPU state has reached the GPU before final J verification completes.

  ); // Finishes updating the intermediate browser status.

} // Finishes starting GPU J runtime verification.