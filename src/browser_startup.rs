/*
Purpose: contain the browser and WebAssembly startup orchestration.

This module will keep browser startup, WebGPU orchestration, and DOM status
handling out of `lib.rs` so the crate root remains easy to understand.
*/

use crate::gpu; // Gives this browser-startup module access to the crate's WebGPU modules.

use crate::physics; // Gives this browser-startup module access to the CPU-side physics reference code.

use crate::state; // Gives this browser-startup module access to the Majorana state type.

use wasm_bindgen::prelude::*; // Imports the wasm-bindgen tools required for the browser WebAssembly startup function.

use wasm_bindgen_futures::spawn_local; // Imports the helper used to start asynchronous browser work without blocking the page.


#[wasm_bindgen(start)] // Tells wasm-bindgen to run the next function automatically when the WebAssembly module starts.

pub fn start() -> Result<(), JsValue> { // Defines the startup function and says it can either succeed or return a JavaScript-compatible error.
  
  console_error_panic_hook::set_once(); // Installs a panic hook so Rust errors are easier to read in the browser console.
  
  let window = web_sys::window().expect("Could not get browser window"); // Gets the browser's Window object and stops with this message if it is unavailable.
  
  let document = window.document().expect("Could not get document"); // Gets the HTML document loaded inside the browser window and stops if it is unavailable.
  
  let status = document.get_element_by_id("status").expect("Could not find element with id=status"); // Finds the HTML element whose id is "status" and stops if that element does not exist.

  status.set_text_content(Some("Rust/WASM loaded successfully.")); // Replaces the text inside the HTML status element with a success message.

  spawn_local(async move { // Starts an asynchronous Rust task where our WebGPU initialization will run.

    let adapter = gpu::gpu_context::request_adapter().await; // Delegates WebGPU adapter discovery to the explicitly named GPU context module.

    status.set_text_content(Some("WebGPU adapter acquired successfully.")); // Changes the webpage status only after Rust successfully receives a compatible GPU adapter.

    let (device, queue) = gpu::gpu_context::request_device_and_queue(&adapter).await; // Delegates WebGPU device and queue creation to the explicitly named GPU context module.

    let apply_j_shader = gpu::shaders::create_apply_j_shader(&device); // Creates the WGSL shader module and keeps its handle so the compute pipeline can use it.

    let apply_j_pipeline = gpu::pipelines::create_apply_j_pipeline( // Creates the J compute pipeline and keeps its handle so we can connect GPU resources to it.

      &device, // Gives the pipeline helper access to the existing WebGPU device.

      &apply_j_shader, // Gives the pipeline helper the already-created Apply J shader module.

    ); // Finishes the compute-pipeline creation call.

    status.set_text_content(Some("WebGPU device and queue created successfully.")); // Updates the page only after Rust successfully creates the GPU device and command queue.

    let majorana_state = state::MajoranaState::new(); // Creates the initial four-component state through our tested production `MajoranaState` API instead of duplicating the raw array here.

    let majorana_components = majorana_state.components(); // Borrows the tested four-f32 component array so the WebGPU code can upload the same state representation.

    let buffer_size = std::mem::size_of_val(majorana_components) as wgpu::BufferAddress; // Calculates the number of bytes needed to store one four-component Majorana state on the GPU.

    let (state_buffer, readback_buffer) = gpu::state_buffers::create_majorana_buffers(&device, buffer_size); // Creates the Majorana state and readback GPU buffers through the explicitly named state buffer module.

    let apply_j_bind_group = gpu::bind_groups::create_apply_j_bind_group( // Creates the resource connection between the existing state buffer and the J compute pipeline so the compute pass can use it.

      &device, // Gives the helper access to the WebGPU device.

      &apply_j_pipeline, // Gives the helper the compute pipeline whose binding layout must be satisfied.

      &state_buffer, // Connects the existing sixteen-byte Majorana state buffer to shader binding zero.

    ); // Finishes the bind-group creation call.

    let mut encoder = gpu::commands::create_command_encoder(&device); // Creates the GPU command encoder through the dedicated command module instead of exposing that low-level setup in `lib.rs`.

    gpu::commands::record_apply_j( // Delegates recording of the Apply J compute pass to the dedicated GPU command module.

      &mut encoder, // Gives the helper mutable access to the existing command encoder.

      &apply_j_pipeline, // Gives the helper the existing Apply J compute pipeline.

      &apply_j_bind_group, // Gives the helper the existing bind group connected to the Majorana state buffer.

    ); // Finishes recording the Apply J compute commands.

    gpu::commands::record_readback_copy( // Delegates recording of the GPU-to-readback buffer copy to the dedicated command module.

      &mut encoder, // Gives the helper mutable access to the existing command encoder.

      &state_buffer, // Gives the helper the GPU buffer containing the transformed Majorana state.

      &readback_buffer, // Gives the helper the CPU-readable staging buffer that will receive the copied bytes.

      buffer_size, // Copies exactly the sixteen bytes already calculated for one Majorana state.

    ); // Finishes recording the readback-copy command.

    queue.write_buffer(&state_buffer, 0, bytemuck::cast_slice(majorana_components)); // Converts the tested f32 components into bytes and uploads those exact bytes into the GPU buffer.

    gpu::commands::submit_commands( // Delegates finishing and submitting the recorded GPU command buffer to the dedicated command module.

      &queue, // Gives the helper access to the existing WebGPU command queue.

      encoder, // Transfers ownership of the completed command encoder so the helper can finish and submit it.

    ); // Finishes the command-submission call.

    let readback_slice = readback_buffer.slice(..); // Creates a view covering the entire readback buffer so we can request CPU-readable mapping for all sixteen bytes.

    let readback_buffer_for_callback = readback_buffer.clone(); // Clones the lightweight wgpu buffer handle so the asynchronous mapping callback can safely access the same underlying GPU buffer later.

    let expected_components = physics::j::apply_j(majorana_components); // Computes the independently tested CPU J result that the GPU-transformed readback state must match.

    let status_for_readback = status.clone(); // Clones the browser status-element handle so the asynchronous readback callback can update the webpage later.

    readback_slice.map_async( // Asynchronously asks WebGPU to make the copied readback-buffer bytes accessible to CPU code.

      wgpu::MapMode::Read, move |map_result| { // Requests read-only mapping and starts a callback that will run after WebGPU finishes preparing the buffer.

        match map_result { // Examines whether WebGPU successfully mapped the readback buffer or returned an error.

          Ok(()) => { // Starts the success branch, which runs only when the readback buffer is ready for CPU access.

            let mapped_range = readback_buffer_for_callback.slice(..).get_mapped_range().expect("Could not access mapped readback bytes"); // Gets the mapped byte view and stops with a clear error if wgpu cannot provide access to that mapped range.

            let reconstructed_state = state::MajoranaState::from_bytes(&mapped_range); // Uses our tested byte-conversion constructor to rebuild the four-component Majorana state from the GPU readback bytes.

            if reconstructed_state.components() == &expected_components { // Checks whether all four values reconstructed from GPU memory exactly match the four values originally uploaded.

              status_for_readback.set_text_content(Some("GPU J operation verified against CPU reference.")); // Reports success only after the GPU-computed J result exactly matches the independently computed CPU reference.

            } else { // Starts the failure branch when the four GPU-readback components do not exactly match the expected CPU components.

              status_for_readback.set_text_content(Some("GPU round-trip verification failed: readback values did not match the uploaded Majorana state.")); // Shows a visible error message when the GPU data round trip changes or corrupts any component.

            } // Closes the success-versus-failure comparison of the GPU readback values.

            drop(mapped_range); // Explicitly releases the CPU-readable mapped byte view before we ask WebGPU to unmap the underlying buffer.

            readback_buffer_for_callback.unmap(); // Releases the CPU mapping now that the readback bytes have been copied and are no longer being accessed.

          } // Closes the successful `Ok(())` mapping branch.

          Err(map_error) => { // Starts the failure branch when WebGPU cannot make the readback buffer accessible to CPU code.

            status_for_readback.set_text_content(Some(&format!("GPU readback mapping failed: {map_error:?}"))); // Displays the mapping error in the browser so the failure is visible instead of silently disappearing.

          } // Closes the `Err(map_error)` branch that handles WebGPU mapping failures.

        } // Closes the `match map_result` statement after both success and failure cases have been handled.

      } // Closes the asynchronous mapping callback that handles successful or failed GPU readback mapping.

    ); // Finishes the `map_async` call that requested CPU-readable access to the readback buffer.

    status.set_text_content(Some("Majorana state uploaded to GPU successfully.")); // Updates the webpage only after the four-component CPU state has been written into the GPU buffer.   

  }); // Closes the async block and finishes the spawn_local function call.

  Ok(()) // Tells Rust that the outer start function finished successfully.

} // Closes the outer start function.