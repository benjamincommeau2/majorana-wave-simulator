/*
Purpose: contain the browser and WebAssembly startup orchestration.

This module will keep browser startup, WebGPU orchestration, and DOM status
handling out of `lib.rs` so the crate root remains easy to understand.
*/

use crate::gpu; // Gives this browser-startup module access to the crate's WebGPU modules.

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

    gpu::start_gpu_j_runtime_verification::start_gpu_j_runtime_verification( // Delegates the complete GPU J dispatch, readback, and CPU-reference comparison to its explicitly named module.

      &device, // Supplies the existing WebGPU device without transferring ownership needed later by the cube renderer.

      &queue, // Supplies the existing WebGPU queue without transferring ownership needed later by the cube renderer.

      &status, // Supplies the browser status element so verification progress and results remain visible.

    ); // Finishes starting GPU J runtime verification.

    #[cfg(target_arch = "wasm32")] // Starts the visual development-cube animation only in the browser WebAssembly build.

      gpu::development_cube::start_mouse_rotation( // Starts the development cube with click-hold-drag rotation instead of automatic spinning.

      _render_canvas, // Gives the interaction module ownership of the browser canvas so it can listen for mouse dragging.

      surface, // Gives the animation loop ownership of the configured browser rendering surface.

      device, // Gives the animation loop ownership of the existing WebGPU device.

      queue, // Gives the animation loop ownership of the existing WebGPU queue.

      development_cube_pipeline, // Gives the animation loop ownership of the already-created cube rendering pipeline.

    ); // Finishes starting the continuously rotating development cube.

  }); // Closes the async block and finishes the spawn_local function call.

  Ok(()) // Tells Rust that the outer start function finished successfully.

} // Closes the outer start function.