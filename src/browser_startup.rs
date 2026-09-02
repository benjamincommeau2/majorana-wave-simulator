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

  status.set_text_content(

    Some(

      "Rust/WASM loaded. Checking WebGPU support...",

    ),

  );

  let canvas = document.get_element_by_id("render-canvas").expect("Could not find element with id=render-canvas"); // Finds the dedicated browser canvas that future WebGPU rendering will target.

  let _render_canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Could not convert render-canvas into an HtmlCanvasElement"); // Converts the generic browser element into the specific HTML canvas type required by WebGPU while allowing native tests to treat the browser-only handle as intentionally unused.

  let query_string = window

    .location()

    .search()

    .unwrap_or_default();


  let simulate_webgpu_unavailable =
    crate::webgpu_compatibility::should_simulate_webgpu_unavailable(

      &query_string,

    );

  spawn_local(async move { // Starts an asynchronous Rust task where our WebGPU initialization will run.

    if simulate_webgpu_unavailable {

      show_webgpu_unavailable(

        &status,

        &_render_canvas,

        true,

      );


      return;

    }

    let instance = gpu::gpu_context::create_instance(); // Creates the shared WebGPU instance that will later also own the rendering-surface creation step.

    #[cfg(target_arch = "wasm32")] // Includes this browser-canvas surface code only when compiling the application for WebAssembly.

    let surface = instance.create_surface( // Asks the shared WebGPU instance to create a presentable rendering surface for our browser canvas.

      wgpu::SurfaceTarget::Canvas(_render_canvas.clone()), // Supplies the verified typed HTML canvas handle that wgpu requires for browser surface creation.

    ).expect("Could not create WebGPU surface from render-canvas"); // Stops with a clear error if the browser canvas cannot become a WebGPU rendering surface.

    let adapter =

      match gpu::gpu_context::try_request_adapter(

        &instance,

      )

      .await

      {

        Ok(adapter) => {

          adapter

        }


        Err(_error) => {

          show_webgpu_unavailable(

            &status,

            &_render_canvas,

            false,

          );


          return;

        }

      };


    status.set_text_content(

      Some(

        "WebGPU adapter acquired successfully.",

      ),

    );

    let (

      device,

      queue,

    ) =

      match gpu::gpu_context::try_request_device_and_queue(

        &adapter,

      )

      .await

      {

        Ok(device_and_queue) => {

          device_and_queue

        }


        Err(error) => {

          let error_message = error.to_string();


          show_webgpu_device_failure(

            &status,

            &_render_canvas,

            &error_message,

          );


          return;

        }

      };

    let spatial_majorana_field = crate::spatial_majorana_field::SpatialMajoranaField::new_centered_gaussian( // Creates the actual three-dimensional Majorana field that will be uploaded to WebGPU.

      16, // Creates sixteen spatial samples along each of the x, y, and z axes for a total of 4096 field points.

      2.0, // Uses the currently tested Gaussian width for this first spatial visualization.

    ); // Finishes creating the initial spatial Majorana field.

    let spatial_majorana_field_bytes = bytemuck::cast_slice( // Views the contiguous four-component field points as raw bytes without copying them.

      spatial_majorana_field.points(), // Borrows all 4096 four-component Majorana field points in their existing contiguous memory layout.

    ); // Finishes creating the byte view used for GPU upload.

    let spatial_majorana_field_buffer = gpu::spatial_majorana_field_buffer::create_spatial_majorana_field_buffer( // Allocates GPU storage large enough to hold the complete three-dimensional Majorana field.

      &device, // Supplies the existing WebGPU device that owns the new field buffer.

      spatial_majorana_field_bytes.len() as wgpu::BufferAddress, // Allocates exactly the number of bytes occupied by all 4096 four-component field points.

    ); // Finishes creating the GPU spatial-field buffer.

    queue.write_buffer( // Uploads the initialized spatial Majorana field into the newly allocated GPU storage buffer.

      &spatial_majorana_field_buffer, // Selects the GPU storage buffer reserved for the complete spatial field.

      0, // Starts writing at byte zero because this upload replaces the complete initial contents of the field buffer.

      spatial_majorana_field_bytes, // Supplies the actual Gaussian Majorana field data as contiguous bytes.

    ); // Finishes scheduling the spatial-field upload.

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

    #[cfg(target_arch = "wasm32")] // Includes spatial-field shader creation only in the browser WebAssembly build.

    let spatial_majorana_field_shader = gpu::shader_modules::create_spatial_majorana_field_shader( // Compiles the rendering shader that reads the actual uploaded spatial Majorana field.

      &device, // Supplies the existing WebGPU device.

    ); // Finishes compiling the spatial-field shader.

    #[cfg(target_arch = "wasm32")] // Includes spatial-field pipeline creation only in the browser WebAssembly build.

    let spatial_majorana_field_pipeline = gpu::pipelines::create_spatial_majorana_field_pipeline( // Creates the triangle-based pipeline used to render the actual field samples.

      &device, // Supplies the existing WebGPU device.

      &spatial_majorana_field_shader, // Supplies the compiled field-rendering WGSL module.

      surface_config.format, // Matches the field renderer output to the browser surface format.

    ); // Finishes creating the spatial Majorana field rendering pipeline.

    gpu::start_gpu_j_runtime_verification::start_gpu_j_runtime_verification( // Delegates the complete GPU J dispatch, readback, and CPU-reference comparison to its explicitly named module.

      &device, // Supplies the existing WebGPU device without transferring ownership needed later by the cube renderer.

      &queue, // Supplies the existing WebGPU queue without transferring ownership needed later by the cube renderer.

      &status, // Supplies the browser status element so verification progress and results remain visible.

    ); // Finishes starting GPU J runtime verification.

    #[cfg(target_arch = "wasm32")] // Starts the visual development-cube animation only in the browser WebAssembly build.

    crate::browser_canvas_recorder::attach_canvas_recorder(

      &_render_canvas,

    )

    .expect(

      "Could not attach the LinkedIn canvas recorder",

    );

      #[cfg(target_arch = "wasm32")]
      
      gpu::development_cube::start_mouse_rotation( // Starts the development cube with click-hold-drag rotation instead of automatic spinning.

      _render_canvas, // Gives the interaction module ownership of the browser canvas so it can listen for mouse dragging.

      surface, // Gives the animation loop ownership of the configured browser rendering surface.

      device, // Gives the animation loop ownership of the existing WebGPU device.

      queue, // Gives the animation loop ownership of the existing WebGPU queue.

      spatial_majorana_field_pipeline, // Gives the animation loop ownership of the actual spatial-field rendering pipeline.

      spatial_majorana_field_buffer, // Gives the animation loop ownership of the GPU buffer containing all 4096 Majorana field samples.

    ); // Finishes starting the continuously rotating development cube.

  }); // Closes the async block and finishes the spawn_local function call.

  Ok(()) // Tells Rust that the outer start function finished successfully.

} // Closes the outer start function.

fn show_webgpu_unavailable(

  status: &web_sys::Element,

  canvas: &web_sys::HtmlCanvasElement,

  simulated_failure: bool,

) {

  let simulated_notice =

    if simulated_failure {

      "\n\nDevelopment test: simulated WebGPU failure."

    } else {

      ""

    };


  let message = format!(

    concat!(

      "WebGPU unavailable\n\n",

      "This simulator requires WebGPU hardware acceleration ",

      "for its GPU physics and rendering.\n\n",

      "Your browser did not provide a compatible WebGPU adapter.\n\n",

      "Try:\n",

      "- Enable hardware acceleration in your browser\n",

      "- Restart the browser after changing that setting\n",

      "- Update to a current WebGPU-capable browser\n",

      "- Check your browser's GPU/WebGPU support\n\n",

      "Your GPU hardware may still be compatible even if the ",

      "current browser configuration does not expose WebGPU.",

      "{}",

    ),

    simulated_notice,

  );


  status.set_text_content(

    Some(

      &message,

    ),

  );


  let _ = canvas.set_attribute(

    "hidden",

    "",

  );


  hide_simulator_interface();

}


fn show_webgpu_device_failure(

  status: &web_sys::Element,

  canvas: &web_sys::HtmlCanvasElement,

  error: &str,

) {

  let message = format!(

    concat!(

      "WebGPU device creation failed\n\n",

      "A WebGPU adapter was found, but the simulator could not ",

      "create the GPU device required for physics and rendering.\n\n",

      "Try enabling browser hardware acceleration, updating your ",

      "browser and graphics drivers, then restarting the browser.\n\n",

      "Technical detail: {}",

    ),

    error,

  );


  status.set_text_content(

    Some(

      &message,

    ),

  );


  let _ = canvas.set_attribute(

    "hidden",

    "",

  );


  hide_simulator_interface();

}


fn hide_simulator_interface() {

  let Some(window) = web_sys::window()

  else {

    return;

  };


  let Some(document) = window.document()

  else {

    return;

  };


  if let Some(controls) = document.get_element_by_id(

    "simulator-controls",

  ) {

    let _ = controls.set_attribute(

      "hidden",

      "",

    );

  }


  if let Some(instructions) = document.get_element_by_id(

    "interaction-instructions",

  ) {

    let _ = instructions.set_attribute(

      "hidden",

      "",

    );

  }

}