#[cfg(any(target_arch = "wasm32", test))] // Compiles mouse rotation for the browser and for native unit tests, but not for unused ordinary native builds.

mod mouse_drag_rotation; // Keeps mouse interaction private while allowing its deterministic behavior to be unit tested.

#[cfg(target_arch = "wasm32")] // Includes one-frame spatial Majorana field rendering only in the browser build.

mod render_spatial_majorana_field_frame; // Keeps the GPU commands for drawing one actual spatial-field frame separate from animation setup.

#[cfg(target_arch = "wasm32")] // Includes browser animation support only when compiling for WebAssembly.

use std::cell::RefCell; // Provides interior mutability so the animation callback can store a reference to itself.

#[cfg(target_arch = "wasm32")] // Includes reference-counting support only in the browser build.

use std::rc::Rc; // Lets the requestAnimationFrame callback remain alive across many browser frames.

#[cfg(target_arch = "wasm32")] // Includes the JavaScript callback wrapper only in the browser build.

use wasm_bindgen::closure::Closure; // Wraps our Rust animation function so the browser can call it repeatedly.

#[cfg(target_arch = "wasm32")] // Includes browser type-casting support only in the WebAssembly build.

use wasm_bindgen::JsCast; // Lets the Rust Closure be supplied where requestAnimationFrame expects a JavaScript function.

#[cfg(target_arch = "wasm32")] // Builds this animation function only for the browser target.

pub fn start_mouse_rotation( // Starts the spatial Majorana field renderer whose orientation is controlled by click-and-drag mouse movement.

  canvas: web_sys::HtmlCanvasElement, // Receives the browser canvas so mouse listeners can control the displayed three-dimensional field.

  surface: wgpu::Surface<'static>, // Takes ownership of the configured browser surface used to present rendered field frames.

  device: wgpu::Device, // Takes ownership of the WebGPU device used to create the field-rendering resources and per-frame commands.

  queue: wgpu::Queue, // Takes ownership of the queue used to update rotation and submit each field-rendering frame.

  pipeline: wgpu::RenderPipeline, // Takes ownership of the already-created spatial Majorana field rendering pipeline.

  spatial_majorana_field_buffer: wgpu::Buffer, // Takes ownership of the GPU storage buffer containing all 4096 four-component spatial field points.

) { // Starts the spatial-field animation setup.

  let rotation_buffer = device.create_buffer( // Creates the small GPU buffer whose angle value will change every animation frame.

    &wgpu::BufferDescriptor { // Starts the rotation-buffer description.

      label: Some("Development Cube Rotation Buffer"), // Gives the rotation buffer an explicit debugging name.

      size: 16, // Allocates sixteen bytes so the uniform has the alignment-friendly size of four f32 values.

      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // Lets the shader read the buffer as a uniform and lets Rust update it through the queue.

      mapped_at_creation: false, // Leaves the buffer unmapped because Queue::write_buffer will update it each frame.

    }, // Finishes the rotation-buffer description.

  ); // Finishes creating the rotation buffer.

  let field_bind_group = crate::gpu::bind_groups::create_spatial_majorana_field_render_bind_group( // Connects both rotation and the actual uploaded field data to the spatial rendering shader.

    &device, // Supplies the WebGPU device that creates the field render bind group.

    &pipeline, // Supplies the spatial-field render pipeline whose binding layout comes from WGSL.

    &rotation_buffer, // Supplies binding zero containing the mouse-controlled yaw and pitch.

    &spatial_majorana_field_buffer, // Supplies binding one containing all 4096 uploaded Majorana field points.

  ); // Finishes creating the spatial-field rendering bind group.

  let drag_state = mouse_drag_rotation::attach_mouse_drag_rotation( // Delegates browser mouse interaction to the explicitly named mouse-drag module.

    &canvas, // Supplies the development canvas on which cube dragging is observed.

  ); // Finishes creating the shared mouse-controlled rotation state.

  let mut simulation_clock = crate::simulation_clock::SimulationClock::new( // Creates the persistent scheduler that separates browser render timing from future fixed physics evolution.

    0.01, // Temporarily uses a 0.01 simulation-time physics step only to exercise browser scheduling; the final numerical physics timestep will be chosen later.

    1.0, // Temporarily requests one simulation-time unit of playback per real-world second.

    4, // Prevents one delayed browser frame from requesting more than four future physics steps.

  ); // Finishes creating the persistent frame-independent simulation clock.

  let animation_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None)); // Creates explicitly typed shared storage for the repeating browser animation callback so Rust knows what value the initially empty Option will later contain.

  let animation_callback_for_start = animation_callback.clone(); // Keeps a second reference so we can schedule the first frame after constructing the callback.

  *animation_callback_for_start.borrow_mut() = Some( // Stores the completed callback inside the shared animation-callback container.

    Closure::<dyn FnMut(f64)>::new(move |timestamp_ms: f64| { // Runs once per browser frame and now uses the browser timestamp to schedule physics independently from rendering.

      let _scheduled_physics_steps = simulation_clock.steps_for_frame( // Asks the persistent simulation clock how many fixed physics steps should occur before this rendered frame.

        timestamp_ms, // Supplies the actual requestAnimationFrame timestamp generated by the browser.

      ); // Finishes calculating this frame's future physics-step count without evolving the field yet.

      let rotation_values = { // Copies the current mouse-controlled orientation into the four-f32 GPU uniform.

        let state = drag_state.borrow(); // Reads the latest mouse-controlled rotation state.

        let [yaw, pitch] = state.angles(); // Requests only the two angles that the renderer needs from the mouse-interaction module.

        [ // Starts the sixteen-byte uniform value sent to WGSL.

          yaw, // Sends the horizontal rotation angle to rotation.x.

          pitch, // Sends the vertical rotation angle to rotation.y.

          0.0, // Leaves the third uniform component unused.

          0.0, // Leaves the fourth uniform component unused.

        ] // Finishes the four-component rotation uniform.

      }; // Finishes copying the mouse-controlled angles.

      render_spatial_majorana_field_frame::render_spatial_majorana_field_frame( // Renders the actual uploaded Majorana field using the current mouse-controlled orientation.

        &surface, // Supplies the configured browser surface that receives this field frame.

        &device, // Supplies the WebGPU device used to record the frame.

        &queue, // Supplies the queue used for rotation updates and rendering submission.

        &pipeline, // Supplies the spatial Majorana field rendering pipeline.

        &rotation_buffer, // Supplies the yaw-and-pitch uniform buffer.

        &field_bind_group, // Supplies both the rotation uniform and complete uploaded field storage buffer.

        &rotation_values, // Supplies the current mouse-controlled yaw and pitch.

      ); // Finishes rendering this spatial Majorana field frame.

      let window = web_sys::window().expect("Could not get browser window for cube animation"); // Gets the browser Window needed to request the next frame.

      window.request_animation_frame( // Asks the browser to call this same Rust callback again for the next display frame.

        animation_callback.borrow().as_ref().expect("Cube animation callback disappeared").as_ref().unchecked_ref(), // Passes the stored Rust callback back to the browser as a JavaScript-compatible function.

      ).expect("Could not request the next cube animation frame"); // Stops clearly if the next animation callback cannot be scheduled.

    }), // Finishes constructing the repeating animation callback.

  ); // Finishes storing the animation callback.

  let window = web_sys::window().expect("Could not get browser window to start cube animation"); // Gets the browser Window used to schedule the first animation frame.

  window.request_animation_frame( // Requests the first frame, after which the callback will continue requesting later frames itself.

    animation_callback_for_start.borrow().as_ref().expect("Cube animation callback was not created").as_ref().unchecked_ref(), // Supplies the newly created Rust animation callback to the browser.

  ).expect("Could not start cube animation"); // Stops clearly if the first animation frame cannot be scheduled.

} // Finishes the development-cube rotation setup.