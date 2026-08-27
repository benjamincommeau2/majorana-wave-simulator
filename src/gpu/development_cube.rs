#[cfg(target_arch = "wasm32")] // Includes browser animation support only when compiling for WebAssembly.

use std::cell::RefCell; // Provides interior mutability so the animation callback can store a reference to itself.

#[cfg(target_arch = "wasm32")] // Includes reference-counting support only in the browser build.

use std::rc::Rc; // Lets the requestAnimationFrame callback remain alive across many browser frames.

#[cfg(target_arch = "wasm32")] // Includes the JavaScript callback wrapper only in the browser build.

use wasm_bindgen::closure::Closure; // Wraps our Rust animation function so the browser can call it repeatedly.

#[cfg(target_arch = "wasm32")] // Includes browser type-casting support only in the WebAssembly build.

use wasm_bindgen::JsCast; // Lets the Rust Closure be supplied where requestAnimationFrame expects a JavaScript function.

#[cfg(target_arch = "wasm32")] // Includes the mouse-drag state only in the browser build.

struct CubeDragState { // Stores the small amount of browser interaction state needed to rotate the development cube.

  yaw: f32, // Stores the current left-right rotation angle.

  pitch: f32, // Stores the current up-down rotation angle.

  dragging: bool, // Records whether the mouse button is currently being held down for cube rotation.

  last_x: i32, // Stores the previous horizontal mouse position using the same integer pixel type returned by MouseEvent::client_x.

  last_y: i32, // Stores the previous vertical mouse position using the same integer pixel type returned by MouseEvent::client_y.

} // Finishes the mouse-drag state type.

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

  let drag_state = Rc::new(RefCell::new( // Creates shared mouse state that can be accessed by all three browser mouse callbacks.

    CubeDragState { // Starts the initial cube orientation and mouse-drag state.

      yaw: 0.65, // Starts with the same useful left-right viewing angle as the earlier static cube.

      pitch: 0.45, // Starts with the same useful vertical tilt as the earlier static cube.

      dragging: false, // Starts with mouse rotation disabled until the user presses the mouse button.

      last_x: 0, // Initializes the previous horizontal mouse position in integer browser pixels.

      last_y: 0, // Initializes the previous vertical mouse position in integer browser pixels.

    }, // Finishes the initial cube-drag state.

  )); // Finishes wrapping the state so several browser callbacks can share and modify it.

  let drag_state_for_down = drag_state.clone(); // Gives the mouse-down callback access to the shared cube-drag state.

  let mouse_down = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Starts the callback that activates cube rotation when the user presses the mouse button.

    let mut state = drag_state_for_down.borrow_mut(); // Borrows the shared drag state mutably so this callback can update it.

    state.dragging = true; // Marks the cube as actively being dragged.

    state.last_x = event.client_x(); // Remembers where the horizontal drag started.

    state.last_y = event.client_y(); // Remembers where the vertical drag started.

  }); // Finishes the mouse-down callback.

  canvas.add_event_listener_with_callback( // Registers the mouse-down callback on the cube canvas.

    "mousedown", // Runs this callback whenever the mouse button is pressed over the canvas.

    mouse_down.as_ref().unchecked_ref(), // Converts the Rust closure into the JavaScript callback type expected by the browser.

  ).expect("Could not register cube mousedown listener"); // Stops clearly if the browser cannot register the mouse-down handler.

  mouse_down.forget(); // Keeps the mouse-down callback alive for the lifetime of this browser page.

  let drag_state_for_move = drag_state.clone(); // Gives the mouse-move callback access to the same shared cube orientation.

  let mouse_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Starts the callback that changes the cube angles while the mouse is dragged.

    let mut state = drag_state_for_move.borrow_mut(); // Borrows the current cube-drag state so the movement can update its angles.

    if state.dragging { // Changes the cube orientation only while the mouse button is being held.

      let delta_x = (event.client_x() - state.last_x) as f32; // Measures how far the mouse moved horizontally since the previous event.

      let delta_y = (event.client_y() - state.last_y) as f32; // Measures how far the mouse moved vertically since the previous event.

      state.yaw += delta_x * 0.01; // Converts horizontal mouse movement into left-right cube rotation.

      state.pitch = (state.pitch + delta_y * 0.01).clamp(-1.4, 1.4); // Converts vertical movement into tilt while preventing the cube from flipping completely over.

      state.last_x = event.client_x(); // Saves the current horizontal position for the next movement event.

      state.last_y = event.client_y(); // Saves the current vertical position for the next movement event.

    } // Finishes the active-drag check.

  }); // Finishes the mouse-move callback.

  canvas.add_event_listener_with_callback( // Registers mouse movement directly on the cube canvas.

    "mousemove", // Runs this callback whenever the pointer moves across the canvas.

    mouse_move.as_ref().unchecked_ref(), // Converts the Rust mouse-move closure into a browser callback.

  ).expect("Could not register cube mousemove listener"); // Stops clearly if the movement listener cannot be registered.

  mouse_move.forget(); // Keeps the mouse-move callback alive for the lifetime of the page.

  let drag_state_for_up = drag_state.clone(); // Gives the mouse-up callback access to the shared dragging flag.

  let mouse_up = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| { // Starts the callback that stops cube rotation when the mouse button is released.

    drag_state_for_up.borrow_mut().dragging = false; // Stops applying mouse movement to the cube orientation.

  }); // Finishes the mouse-up callback.

  web_sys::window().expect("Could not get browser window for cube mouseup listener").add_event_listener_with_callback( // Registers mouse-up on the whole window so releasing outside the canvas still ends the drag.

    "mouseup", // Runs whenever the pressed mouse button is released.

    mouse_up.as_ref().unchecked_ref(), // Converts the Rust mouse-up closure into the browser callback type.

  ).expect("Could not register cube mouseup listener"); // Stops clearly if the browser cannot register the mouse-release handler.

  mouse_up.forget(); // Keeps the mouse-up callback alive for the lifetime of the page.

  let animation_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None)); // Creates explicitly typed shared storage for the repeating browser animation callback so Rust knows what value the initially empty Option will later contain.

  let animation_callback_for_start = animation_callback.clone(); // Keeps a second reference so we can schedule the first frame after constructing the callback.

  *animation_callback_for_start.borrow_mut() = Some( // Stores the completed callback inside the shared animation-callback container.

    Closure::<dyn FnMut(f64)>::new(move |_timestamp_ms: f64| { // Runs once per browser frame while intentionally ignoring time because the cube orientation now comes from mouse input.

      let rotation_values = { // Copies the current mouse-controlled orientation into a four-f32 value suitable for the existing GPU uniform buffer.

        let state = drag_state.borrow(); // Reads the latest yaw and pitch written by the browser mouse callbacks.

        [ // Starts the sixteen-byte uniform value sent to WGSL.

          state.yaw, // Sends the current horizontal rotation angle to rotation.x.

          state.pitch, // Sends the current vertical rotation angle to rotation.y.

          0.0, // Leaves the third uniform component unused.

          0.0, // Leaves the fourth uniform component unused.

        ] // Finishes the four-component rotation uniform.

      }; // Finishes copying the mouse-controlled angles.

      queue.write_buffer( // Updates the GPU rotation value before drawing this frame.

        &rotation_buffer, // Selects the small cube rotation uniform buffer.

        0, // Starts writing at the beginning of the uniform buffer.

        bytemuck::cast_slice(&rotation_values), // Reinterprets the four f32 values as the sixteen bytes required by the GPU buffer.

      ); // Finishes uploading the current rotation angle.

      let surface_texture = match surface.get_current_texture() { // Acquires the browser surface image that this animation frame will render into.

        wgpu::CurrentSurfaceTexture::Success(texture) => texture, // Uses the frame when WebGPU reports normal successful acquisition.

        wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture, // Still uses a valid frame when the surface reports that reconfiguration may eventually be useful.

        surface_error => panic!("Could not acquire cube animation surface texture: {surface_error:?}"), // Stops clearly if a renderable browser frame cannot be acquired.

      }; // Finishes acquiring this animation frame's surface texture.

      let surface_view = surface_texture.texture.create_view( // Creates the texture view that the render pass will use as its color output.

        &wgpu::TextureViewDescriptor::default(), // Uses the complete surface texture with its default view configuration.

      ); // Finishes creating the animation-frame texture view.

      let mut encoder = device.create_command_encoder( // Creates the command encoder used to record this animation frame.

        &wgpu::CommandEncoderDescriptor { // Starts the per-frame command-encoder description.

          label: Some("Development Cube Animation Encoder"), // Gives the animation encoder a readable debugging label.

        }, // Finishes the command-encoder description.

      ); // Finishes creating the per-frame encoder.

      { // Starts a short scope so the render pass ends before command submission.

        let mut render_pass = encoder.begin_render_pass( // Begins recording the commands that draw this cube frame.

          &wgpu::RenderPassDescriptor { // Starts the animation render-pass description.

            label: Some("Development Cube Animation Pass"), // Gives the repeating render pass a readable debugging name.

            color_attachments: &[ // Starts the list of render targets for this frame.

              Some(wgpu::RenderPassColorAttachment { // Uses the browser surface as the single color output.

                view: &surface_view, // Directs rendered pixels into the current browser surface texture.

                depth_slice: None, // Uses no three-dimensional texture slice because the canvas surface is two-dimensional.

                resolve_target: None, // Uses no multisampling resolve texture for this wireframe preview.

                ops: wgpu::Operations { // Describes how the existing surface pixels are handled.

                  load: wgpu::LoadOp::Clear( // Clears the entire frame before drawing the newly rotated cube.

                    wgpu::Color { // Reuses the dark development background color.

                      r: 0.08, // Sets the red background component.

                      g: 0.12, // Sets the green background component.

                      b: 0.20, // Sets the blue background component.

                      a: 1.0, // Keeps the background fully opaque.

                    }, // Finishes the background color.

                  ), // Finishes the clear operation.

                  store: wgpu::StoreOp::Store, // Preserves the completed frame so it can be presented to the browser.

                }, // Finishes the surface color operations.

              }), // Finishes the browser color attachment.

            ], // Finishes the color-target list.

            depth_stencil_attachment: None, // Keeps depth buffering disabled for the current wireframe cube.

            timestamp_writes: None, // Keeps GPU timing instrumentation disabled while establishing animation correctness.

            occlusion_query_set: None, // Uses no visibility-query system for this simple development renderer.

            multiview_mask: None, // Uses one ordinary browser view.

          }, // Finishes the render-pass description.

        ); // Finishes beginning the animation render pass.

        render_pass.set_pipeline(&pipeline); // Selects the existing development-cube rendering pipeline.

        render_pass.set_bind_group( // Supplies the changing rotation uniform required by the cube shader.

          0, // Connects the resource to shader bind group zero.

          &rotation_bind_group, // Supplies the bind group containing the rotation uniform buffer.

          &[], // Uses no dynamic buffer offsets.

        ); // Finishes binding the cube rotation data.

        render_pass.draw(0..24, 0..1); // Draws the same twelve cube edges using the new orientation for this frame.

      } // Ends the animation render pass before submission.

      queue.submit( // Sends this completed animation frame to the GPU.

        Some(encoder.finish()), // Finishes the command encoder and submits its command buffer.

      ); // Finishes submitting the frame.

      queue.present(surface_texture); // Presents the completed frame inside the browser canvas.

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