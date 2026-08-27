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

use wasm_bindgen::JsCast; // Imports the browser type-casting trait needed to convert a generic HTML element into an HtmlCanvasElement.

#[wasm_bindgen(start)] // Tells wasm-bindgen to run the next function automatically when the WebAssembly module starts.

pub fn start() -> Result<(), JsValue> { // Defines the startup function and says it can either succeed or return a JavaScript-compatible error.
  
  console_error_panic_hook::set_once(); // Installs a panic hook so Rust errors are easier to read in the browser console.
  
  let window = web_sys::window().expect("Could not get browser window"); // Gets the browser's Window object and stops with this message if it is unavailable.
  
  let document = window.document().expect("Could not get document"); // Gets the HTML document loaded inside the browser window and stops if it is unavailable.
  
  let status = document.get_element_by_id("status").expect("Could not find element with id=status"); // Finds the HTML element whose id is "status" and stops if that element does not exist.

  status.set_text_content(Some("Rust/WASM loaded successfully.")); // Replaces the text inside the HTML status element with a success message.

  let canvas = document.get_element_by_id("render-canvas").expect("Could not find element with id=render-canvas"); // Finds the dedicated browser canvas that future WebGPU rendering will target.

  let _render_canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Could not convert render-canvas into an HtmlCanvasElement"); // Converts the generic browser element into the specific HTML canvas type required by WebGPU while allowing native tests to treat the browser-only handle as intentionally unused.

  spawn_local(async move { // Starts an asynchronous Rust task where our WebGPU initialization will run.

    let instance = gpu::gpu_context::create_instance(); // Creates the shared WebGPU instance that will later also own the rendering-surface creation step.

    #[cfg(target_arch = "wasm32")] // Includes this browser-canvas surface code only when compiling the application for WebAssembly.

    let surface = instance.create_surface( // Asks the shared WebGPU instance to create a presentable rendering surface for our browser canvas.

      wgpu::SurfaceTarget::Canvas(_render_canvas.clone()), // Supplies the verified typed HTML canvas handle that wgpu requires for browser surface creation.

    ).expect("Could not create WebGPU surface from render-canvas"); // Stops with a clear error if the browser canvas cannot become a WebGPU rendering surface.

    let adapter = gpu::gpu_context::request_adapter(&instance).await; // Requests the same WebGPU adapter as before while keeping the instance available for the future rendering surface.

    status.set_text_content(Some("WebGPU adapter acquired successfully.")); // Changes the webpage status only after Rust successfully receives a compatible GPU adapter.

    let (device, queue) = gpu::gpu_context::request_device_and_queue(&adapter).await; // Delegates WebGPU device and queue creation to the explicitly named GPU context module.

    #[cfg(target_arch = "wasm32")] // Includes browser-surface configuration only when compiling the application for WebAssembly.

    let surface_config = surface.get_default_config( // Asks wgpu to choose a supported default presentation configuration for this surface and adapter.

      &adapter, // Supplies the GPU adapter so wgpu can choose a surface format supported by that adapter.

      _render_canvas.width(), // Uses the canvas's configured pixel width for the GPU presentation surface.

      _render_canvas.height(), // Uses the canvas's configured pixel height for the GPU presentation surface.

    ).expect("Could not create a compatible WebGPU surface configuration"); // Stops clearly if this adapter cannot present images to this canvas surface.

    #[cfg(target_arch = "wasm32")] // Includes the actual surface-configuration call only in the browser WebAssembly build.

    surface.configure( // Initializes the surface so future render passes can acquire presentable textures from it.

      &device, // Supplies the WebGPU device that will create and submit rendering work.

      &surface_config, // Supplies the supported width, height, format, and presentation settings selected above.

    ); // Finishes configuring the browser rendering surface.

    #[cfg(target_arch = "wasm32")] // Includes cube-shader creation only in the browser WebAssembly build.

    let development_cube_shader = gpu::shader_modules::create_development_cube_shader(&device); // Compiles the dedicated development-cube WGSL shader.

    #[cfg(target_arch = "wasm32")] // Includes cube-pipeline creation only in the browser WebAssembly build.

    let development_cube_pipeline = gpu::pipelines::create_development_cube_pipeline( // Creates the pipeline that will turn shader-generated vertices into visible cube edges.

      &device, // Supplies the existing WebGPU device.

      &development_cube_shader, // Supplies the compiled development-cube shader.

      surface_config.format, // Matches the cube output to the browser surface's configured pixel format.

    ); // Finishes creating the cube render pipeline.

    let apply_j_shader = gpu::shader_modules::create_apply_j_shader(&device); // Creates the WGSL shader module through the clearly named shader-module helper.

    let apply_j_pipeline = gpu::pipelines::create_apply_j_pipeline( // Creates the J compute pipeline and keeps its handle so we can connect GPU resources to it.

      &device, // Gives the pipeline helper access to the existing WebGPU device.

      &apply_j_shader, // Gives the pipeline helper the already-created Apply J shader module.

    ); // Finishes the compute-pipeline creation call.

        #[cfg(target_arch = "wasm32")] // Includes this first visible rendering checkpoint only in the browser WebAssembly build.

    let surface_texture = match surface.get_current_texture() { // Requests the next image that WebGPU can render and present inside the browser canvas.

      wgpu::CurrentSurfaceTexture::Success(texture) => texture, // Uses the acquired texture when the surface reports a normal successful frame.

      wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture, // Still uses the frame if WebGPU says the surface works but may eventually benefit from reconfiguration.

      surface_error => panic!("Could not acquire WebGPU surface texture: {surface_error:?}"), // Stops clearly if the browser cannot provide a renderable frame.

    }; // Finishes selecting the surface texture that will become our visible frame.

    #[cfg(target_arch = "wasm32")] // Includes creation of the renderable texture view only in the browser build.

    let surface_view = surface_texture.texture.create_view( // Creates the view that a WebGPU render pass can use as its color output.

      &wgpu::TextureViewDescriptor::default(), // Uses the default view of the complete surface texture for this first rendering checkpoint.

    ); // Finishes creating the render-target view.

    #[cfg(target_arch = "wasm32")] // Includes this rendering command encoder only in the browser build.

    let mut render_encoder = device.create_command_encoder( // Creates a command encoder that will record our first visible rendering commands.

      &wgpu::CommandEncoderDescriptor { // Starts the descriptor for the rendering command encoder.

        label: Some("First Visible Render Encoder"), // Gives the encoder a clear debugging name.

      }, // Finishes the rendering command-encoder descriptor.

    ); // Finishes creating the rendering command encoder.

    #[cfg(target_arch = "wasm32")] // Includes the render pass only in the browser WebAssembly build.

    { // Starts a scope so the render pass ends before the command encoder is submitted.

      let mut render_pass = render_encoder.begin_render_pass( // Begins the render pass and keeps mutable access so we can issue cube drawing commands.

        &wgpu::RenderPassDescriptor { // Starts the description of the render pass.

          label: Some("Canvas Clear Render Pass"), // Gives this first visual render pass a readable debugging name.

          color_attachments: &[ // Starts the list of color outputs written by this render pass.

            Some(wgpu::RenderPassColorAttachment { // Connects the browser surface texture as the color output.

              view: &surface_view, // Selects the surface texture view as the destination for rendered pixels.

              depth_slice: None, // Uses no three-dimensional texture depth slice because the browser canvas is a normal two-dimensional surface.

              resolve_target: None, // Uses no multisample resolve target for this simple first frame.

              ops: wgpu::Operations { // Defines what should happen to the canvas pixels during this render pass.

                load: wgpu::LoadOp::Clear( // Tells WebGPU to replace the entire canvas with one known color.

                  wgpu::Color { // Defines the clear color that will make GPU rendering visibly obvious.

                    r: 0.08, // Sets a small red component.

                    g: 0.12, // Sets a slightly larger green component.

                    b: 0.20, // Sets the strongest blue component.

                    a: 1.0, // Makes the rendered frame fully opaque.

                  }, // Finishes the clear color.

                ), // Finishes the clear operation.

                store: wgpu::StoreOp::Store, // Keeps the cleared pixels so they can be presented to the browser.

              }, // Finishes the color operations.

            }), // Finishes the browser-canvas color attachment.

          ], // Finishes the color-attachment list.

          depth_stencil_attachment: None, // Uses no depth buffer yet because we are not drawing three-dimensional geometry yet.

          timestamp_writes: None, // Keeps GPU timing measurements disabled for this correctness checkpoint.

          occlusion_query_set: None, // Uses no visibility-query system for this simple clear operation.

          multiview_mask: None, // Uses ordinary single-view rendering rather than multiview rendering.

        }, // Finishes the render-pass descriptor.

      ); // Finishes beginning the render pass.

      render_pass.set_pipeline(&development_cube_pipeline); // Selects the development-cube rendering pipeline for this render pass.

      render_pass.draw(0..24, 0..1); // Launches twenty-four shader-generated vertices, producing the cube's twelve line segments.

    } // Ends the render pass before command submission.

    #[cfg(target_arch = "wasm32")] // Includes submission of the visible frame only in the browser build.

    queue.submit( // Sends the recorded rendering commands to the GPU.

      Some(render_encoder.finish()), // Finishes the encoder and submits its completed command buffer.

    ); // Finishes submitting the first rendering commands.

    #[cfg(target_arch = "wasm32")] // Includes browser presentation only in the WebAssembly build.

    queue.present(surface_texture); // Presents the GPU-rendered surface texture so the clear color becomes visible inside the HTML canvas.

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