// src/gpu/development_cube/mouse_drag_rotation.rs

#[cfg(target_arch = "wasm32")] // Includes this browser-only dependency only when compiling the WebAssembly application.

use std::cell::RefCell; // Provides interior mutability so several browser callbacks can update the same mouse-drag state.

#[cfg(target_arch = "wasm32")] // Includes this browser-only dependency only when compiling the WebAssembly application.

use std::rc::Rc; // Lets the browser callbacks and renderer share ownership of the mouse-drag state.

#[cfg(target_arch = "wasm32")] // Includes this browser-only dependency only when compiling the WebAssembly application.

use wasm_bindgen::closure::Closure; // Wraps Rust mouse-event functions so the browser can call them.

#[cfg(target_arch = "wasm32")] // Includes this browser-only dependency only when compiling the WebAssembly application.

use wasm_bindgen::JsCast; // Converts Rust closures into the JavaScript callback type expected by browser event listeners.

pub(super) struct MouseDragRotation { // Stores the browser interaction state that controls the development cube orientation.

  yaw: f32, // Stores the current left-right rotation angle.

  pitch: f32, // Stores the current up-down rotation angle.

  dragging: bool, // Records whether the mouse button is currently being held down for cube rotation.

  last_x: i32, // Stores the previous horizontal mouse position in browser pixels.

  last_y: i32, // Stores the previous vertical mouse position in browser pixels.

} // Finishes the mouse-drag rotation state.

impl MouseDragRotation { // Defines the mouse-rotation behavior independently from browser event objects.

  fn new() -> Self { // Creates the initial development-cube orientation and inactive drag state.

    Self { // Starts the initial mouse-rotation state.

      yaw: 0.65, // Starts with the established left-right viewing angle.

      pitch: 0.45, // Starts with the established vertical viewing angle.

      dragging: false, // Starts with mouse dragging inactive.

      last_x: 0, // Initializes the previous horizontal mouse position.

      last_y: 0, // Initializes the previous vertical mouse position.

    } // Finishes the initial mouse-rotation state.

  } // Finishes creating the initial state.

  fn start_drag(&mut self, x: i32, y: i32) { // Begins a drag from the supplied browser-pixel position.

    self.dragging = true; // Marks subsequent pointer movement as cube rotation.

    self.last_x = x; // Remembers the horizontal starting position.

    self.last_y = y; // Remembers the vertical starting position.

  } // Finishes starting a drag.

  fn drag_to(&mut self, x: i32, y: i32) { // Applies pointer movement to the cube orientation while dragging is active.

    if self.dragging { // Ignores pointer movement when the mouse button is not being held.

      let delta_x = (x - self.last_x) as f32; // Measures horizontal movement since the previous drag position.

      let delta_y = (y - self.last_y) as f32; // Measures vertical movement since the previous drag position.

      self.yaw += delta_x * 0.01; // Converts horizontal movement into left-right rotation.

      self.pitch = (self.pitch + delta_y * 0.01).clamp(-1.4, 1.4); // Converts vertical movement into tilt while preserving the existing pitch limits.

      self.last_x = x; // Saves the current horizontal position for the next drag update.

      self.last_y = y; // Saves the current vertical position for the next drag update.

    } // Finishes the active-drag check.

  } // Finishes applying drag movement.

  fn stop_drag(&mut self) { // Ends the active mouse drag.

    self.dragging = false; // Prevents later pointer movement from rotating the cube until another drag starts.

  } // Finishes stopping the drag.

  pub(super) fn angles(&self) -> [f32; 2] { // Returns only the orientation values required by the renderer.

    [ // Starts the two-angle result.

      self.yaw, // Returns the current horizontal rotation angle.

      self.pitch, // Returns the current vertical rotation angle.

    ] // Finishes the two-angle result.

  } // Finishes reading the current cube angles.

} // Finishes the mouse-drag rotation implementation.

#[cfg(target_arch = "wasm32")] // Connects the tested mouse-rotation state to real browser mouse events only in the WebAssembly build.

pub(super) fn attach_mouse_drag_rotation( // Registers the browser mouse listeners and returns their shared rotation state.

  canvas: &web_sys::HtmlCanvasElement, // Borrows the development canvas on which mouse-down and mouse-move events are observed.

) -> Rc<RefCell<MouseDragRotation>> { // Returns shared state so the render loop can read the current yaw and pitch.

  let drag_state = Rc::new(RefCell::new( // Creates shared ownership of the tested mouse-rotation state for the browser callbacks and renderer.

    MouseDragRotation::new(), // Uses the same tested initialization logic instead of duplicating the initial values here.

  )); // Finishes creating the shared mouse-rotation state.

  let drag_state_for_down = drag_state.clone(); // Gives the mouse-down callback access to the shared drag state.

  let mouse_down = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Starts dragging when the user presses the mouse button over the canvas.

    drag_state_for_down.borrow_mut().start_drag( // Delegates drag initialization to the testable Rust state behavior.

      event.client_x(), // Supplies the browser's current horizontal mouse position.

      event.client_y(), // Supplies the browser's current vertical mouse position.

    ); // Finishes starting the drag.

  }); // Finishes the mouse-down callback.

  canvas.add_event_listener_with_callback( // Registers the mouse-down callback on the development canvas.

    "mousedown", // Runs when the mouse button is pressed over the canvas.

    mouse_down.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mousedown listener"); // Stops clearly if the browser cannot register the listener.

  mouse_down.forget(); // Keeps the callback alive for the lifetime of the browser page.

  let drag_state_for_move = drag_state.clone(); // Gives the mouse-move callback access to the shared drag state.

  let mouse_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Updates cube rotation while the mouse is dragged.

    drag_state_for_move.borrow_mut().drag_to( // Delegates movement arithmetic and pitch clamping to the testable Rust state behavior.

      event.client_x(), // Supplies the new horizontal mouse position.

      event.client_y(), // Supplies the new vertical mouse position.

    ); // Finishes applying this drag movement.
    
  }); // Finishes the mouse-move callback.

  canvas.add_event_listener_with_callback( // Registers mouse movement directly on the development canvas.

    "mousemove", // Runs whenever the pointer moves across the canvas.

    mouse_move.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mousemove listener"); // Stops clearly if the browser cannot register the listener.

  mouse_move.forget(); // Keeps the callback alive for the lifetime of the browser page.

  let drag_state_for_up = drag_state.clone(); // Gives the mouse-up callback access to the shared dragging flag.

  let mouse_up = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| { // Stops dragging when the mouse button is released.

    drag_state_for_up.borrow_mut().stop_drag(); // Delegates drag termination to the testable Rust state behavior.

  }); // Finishes the mouse-up callback.

  web_sys::window().expect("Could not get browser window for cube mouseup listener").add_event_listener_with_callback( // Registers mouse-up on the window so releasing outside the canvas still ends the drag.

    "mouseup", // Runs whenever the pressed mouse button is released.

    mouse_up.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mouseup listener"); // Stops clearly if the browser cannot register the listener.

  mouse_up.forget(); // Keeps the callback alive for the lifetime of the browser page.

  drag_state // Returns the shared rotation state so the render loop can read its current angles.

} // Finishes registering development-cube mouse rotation.

#[cfg(test)] // Includes the centralized mouse-rotation unit tests only while running cargo test.

#[path = "../../../tests/unit/mouse_drag_rotation_tests.rs"] // Keeps the test source physically centralized under the repository's tests directory.

mod mouse_drag_rotation_tests; // Attaches the centralized tests as a child module so they can verify private rotation behavior.