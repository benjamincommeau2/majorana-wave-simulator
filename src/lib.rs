/*
This file is the main Rust entry point for the browser application.

Its responsibilities include:
- Starting the Rust/WebAssembly application.
- Connecting Rust to browser APIs through wasm-bindgen and web-sys.
- Initializing WebGPU through wgpu.
- Requesting the GPU adapter, device, and command queue.
- Creating and managing GPU resources.
- Coordinating the browser-facing parts of the Majorana wave simulator.

Physics and CPU-side state logic should be kept in separate modules when possible
so that it can be tested independently with `cargo test`.
*/

use wasm_bindgen::prelude::*; // Imports the common wasm-bindgen tools we need to connect Rust with WebAssembly.

use wasm_bindgen_futures::spawn_local; // Imports the helper that lets browser-based Rust start asynchronous work without blocking the webpage.

pub mod state; // Makes the state module part of the crate's public API so integration tests in tests/ can access it.

pub mod gpu; // Makes the `src/gpu/` module available to the rest of the crate.

#[wasm_bindgen(start)] // Tells wasm-bindgen to run the next function automatically when the WebAssembly module starts.

pub fn start() -> Result<(), JsValue> { // Defines the startup function and says it can either succeed or return a JavaScript-compatible error.
  
  console_error_panic_hook::set_once(); // Installs a panic hook so Rust errors are easier to read in the browser console.
  
  let window = web_sys::window().expect("Could not get browser window"); // Gets the browser's Window object and stops with this message if it is unavailable.
  
  let document = window.document().expect("Could not get document"); // Gets the HTML document loaded inside the browser window and stops if it is unavailable.
  
  let status = document.get_element_by_id("status").expect("Could not find element with id=status"); // Finds the HTML element whose id is "status" and stops if that element does not exist.

  status.set_text_content(Some("Rust/WASM loaded successfully.")); // Replaces the text inside the HTML status element with a success message.

  spawn_local(async move { // Starts an asynchronous Rust task where our WebGPU initialization will run.

    let adapter = gpu::context::request_adapter().await; // Delegates WebGPU adapter discovery to the dedicated GPU context module.

    status.set_text_content(Some("WebGPU adapter acquired successfully.")); // Changes the webpage status only after Rust successfully receives a compatible GPU adapter.

    let (device, queue) = gpu::context::request_device_and_queue(&adapter).await; // Delegates WebGPU device and queue creation to the dedicated GPU context module.

    status.set_text_content(Some("WebGPU device and queue created successfully.")); // Updates the page only after Rust successfully creates the GPU device and command queue.

    let majorana_state = state::MajoranaState::new(); // Creates the initial four-component state through our tested production `MajoranaState` API instead of duplicating the raw array here.

    let majorana_components = majorana_state.components(); // Borrows the tested four-f32 component array so the WebGPU code can upload the same state representation.

    let buffer_size = std::mem::size_of_val(majorana_components) as wgpu::BufferAddress; // Calculates the number of bytes needed to store one four-component Majorana state on the GPU.

    let (state_buffer, readback_buffer) = gpu::buffers::create_majorana_buffers(&device, buffer_size); // Creates both GPU buffers through the dedicated buffer module instead of exposing their low-level configuration here.

    let mut encoder = device.create_command_encoder( // Creates a command encoder that will record GPU operations before they are submitted to the GPU. Mut stands for mutable.

      &wgpu::CommandEncoderDescriptor { // Starts the configuration for the command encoder.

        label: Some("Majorana Readback Encoder"), // Gives the encoder a readable debugging label.

    }); // Finishes the create_command_encoder call and stores the encoder in a mutable variable.

    encoder.copy_buffer_to_buffer( // Records a command that will copy the Majorana state from the GPU working buffer into the CPU-readable staging buffer.

      &state_buffer, // Uses the existing GPU Majorana state buffer as the source of the copy.

      0, // Starts reading from byte offset 0 at the beginning of the source buffer.

      &readback_buffer, // Uses the readback staging buffer as the destination of the copy.

      0, // Starts writing at byte offset 0 at the beginning of the destination buffer.

      std::mem::size_of_val(majorana_components) as wgpu::BufferAddress // Copies exactly the number of bytes occupied by the four-f32 Majorana state.

    ); // Finishes recording the buffer-to-buffer copy command.

    queue.write_buffer(&state_buffer, 0, bytemuck::cast_slice(majorana_components)); // Converts the tested f32 components into bytes and uploads those exact bytes into the GPU buffer.

    queue.submit(Some(encoder.finish())); // Finishes recording the encoder and submits its completed command buffer to the GPU queue for execution. In Rust, Some is a variant of the Option enum that is used to wrap a value when a value is successfully present. Because Rust does not have a null value, it uses Option to safely represent either the existence of data (Some(value)) or its absence (None).

    let readback_slice = readback_buffer.slice(..); // Creates a view covering the entire readback buffer so we can request CPU-readable mapping for all sixteen bytes.

    let readback_buffer_for_callback = readback_buffer.clone(); // Clones the lightweight wgpu buffer handle so the asynchronous mapping callback can safely access the same underlying GPU buffer later.

    let expected_components = *majorana_components; // Copies the four expected f32 values into an owned array that can safely move into the asynchronous readback callback.

    let status_for_readback = status.clone(); // Clones the browser status-element handle so the asynchronous readback callback can update the webpage later.

    readback_slice.map_async( // Asynchronously asks WebGPU to make the copied readback-buffer bytes accessible to CPU code.

      wgpu::MapMode::Read, move |map_result| { // Requests read-only mapping and starts a callback that will run after WebGPU finishes preparing the buffer.

        match map_result { // Examines whether WebGPU successfully mapped the readback buffer or returned an error.

          Ok(()) => { // Starts the success branch, which runs only when the readback buffer is ready for CPU access.

            let mapped_range = readback_buffer_for_callback.slice(..).get_mapped_range().expect("Could not access mapped readback bytes"); // Gets the mapped byte view and stops with a clear error if wgpu cannot provide access to that mapped range.

            let reconstructed_state = state::MajoranaState::from_bytes(&mapped_range); // Uses our tested byte-conversion constructor to rebuild the four-component Majorana state from the GPU readback bytes.

            if reconstructed_state.components() == &expected_components { // Checks whether all four values reconstructed from GPU memory exactly match the four values originally uploaded.

              status_for_readback.set_text_content(Some("GPU round-trip verified: [1.0, 0.0, 0.0, 0.0].")); // Shows a visible success message only when the GPU-readback state matches the expected Majorana state.

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