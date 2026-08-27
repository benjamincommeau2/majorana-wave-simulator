#[cfg(target_arch = "wasm32")] // Includes browser mouse-rotation support only when compiling for WebAssembly.

mod mouse_drag_rotation; // Keeps development-cube mouse interaction in a separately named module.

#[cfg(target_arch = "wasm32")] // Includes one-frame development-cube rendering only in the browser build.

mod render_development_cube_frame; // Keeps the GPU commands for rendering one cube frame in a separately named module.

#[cfg(target_arch = "wasm32")] // Includes browser animation support only when compiling for WebAssembly.

use std::cell::RefCell; // Provides interior mutability so the animation callback can store a reference to itself.

#[cfg(target_arch = "wasm32")] // Includes reference-counting support only in the browser build.

use std::rc::Rc; // Lets the requestAnimationFrame callback remain alive across many browser frames.

#[cfg(target_arch = "wasm32")] // Includes the JavaScript callback wrapper only in the browser build.

use wasm_bindgen::closure::Closure; // Wraps our Rust animation function so the browser can call it repeatedly.

#[cfg(target_arch = "wasm32")] // Includes browser type-casting support only in the WebAssembly build.

use wasm_bindgen::JsCast; // Lets the Rust Closure be supplied where requestAnimationFrame expects a JavaScript function.

#[cfg(target_arch = "wasm32")] // Builds this animation function only for the browser target.

pub fn start_mouse_rotation( // Starts the development-cube renderer whose orientation is controlled by click-and-drag mouse movement.

  canvas: web_sys::HtmlCanvasElement, // Receives the browser canvas so mouse-down and mouse-move listeners can be attached directly to the rendered cube.

  surface: wgpu::Surface<'static>, // Takes ownership of the configured browser surface used to present rendered frames.

  device: wgpu::Device, // Takes ownership of the WebGPU device needed to create per-frame rendering commands.

  queue: wgpu::Queue, // Takes ownership of the queue used to update the angle and submit each rendered frame.

  pipeline: wgpu::RenderPipeline, // Takes ownership of the already-created development-cube rendering pipeline.

) { // Starts the development-cube animation setup.

  let rotation_buffer = device.create_buffer( // Creates the small GPU buffer whose angle value will change every animation frame.

    &wgpu::BufferDescriptor { // Starts the rotation-buffer description.

      label: Some("Development Cube Rotation Buffer"), // Gives the rotation buffer an explicit debugging name.

      size: 16, // Allocates sixteen bytes so the uniform has the alignment-friendly size of four f32 values.

      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // Lets the shader read the buffer as a uniform and lets Rust update it through the queue.

      mapped_at_creation: false, // Leaves the buffer unmapped because Queue::write_buffer will update it each frame.

    }, // Finishes the rotation-buffer description.

  ); // Finishes creating the rotation buffer.

  let rotation_layout = pipeline.get_bind_group_layout(0); // Retrieves the bind-group layout inferred from the shader's group-zero rotation uniform.

  let rotation_bind_group = device.create_bind_group( // Connects the rotation buffer to the uniform declared in the cube shader.

    &wgpu::BindGroupDescriptor { // Starts the cube rotation bind-group description.

      label: Some("Development Cube Rotation Bind Group"), // Gives the bind group a readable debugging name.

      layout: &rotation_layout, // Uses the layout inferred from the development-cube render pipeline.

      entries: &[ // Starts the list of resources supplied to the cube shader.

        wgpu::BindGroupEntry { // Starts the shader resource at binding zero.

          binding: 0, // Matches binding zero in development_cube.wgsl.

          resource: rotation_buffer.as_entire_binding(), // Makes the complete sixteen-byte rotation buffer visible to the vertex shader.

        }, // Finishes the binding-zero entry.

      ], // Finishes the bind-group resource list.

    }, // Finishes the bind-group description.

  ); // Finishes creating the rotation bind group.

  let drag_state = mouse_drag_rotation::attach_mouse_drag_rotation( // Delegates browser mouse interaction to the explicitly named mouse-drag module.

    &canvas, // Supplies the development canvas on which cube dragging is observed.

  ); // Finishes creating the shared mouse-controlled rotation state.

  let animation_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None)); // Creates explicitly typed shared storage for the repeating browser animation callback so Rust knows what value the initially empty Option will later contain.

  let animation_callback_for_start = animation_callback.clone(); // Keeps a second reference so we can schedule the first frame after constructing the callback.

  *animation_callback_for_start.borrow_mut() = Some( // Stores the completed callback inside the shared animation-callback container.

    Closure::<dyn FnMut(f64)>::new(move |_timestamp_ms: f64| { // Runs once per browser frame while intentionally ignoring time because the cube orientation now comes from mouse input.

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

      render_development_cube_frame::render_development_cube_frame( // Delegates all GPU work for this single frame to the explicitly named frame-rendering module.

        &surface, // Supplies the configured browser surface that receives the rendered frame.

        &device, // Supplies the WebGPU device used to create this frame's command encoder.

        &queue, // Supplies the queue used for the uniform update, GPU submission, and presentation.

        &pipeline, // Supplies the existing development-cube rendering pipeline.

        &rotation_buffer, // Supplies the uniform buffer containing the cube orientation.

        &rotation_bind_group, // Supplies the bind group that exposes the rotation uniform to WGSL.

        &rotation_values, // Supplies the current mouse-controlled yaw and pitch values.

      ); // Finishes rendering this development-cube frame.

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