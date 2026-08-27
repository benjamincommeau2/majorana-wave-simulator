// src/gpu/development_cube/mouse_drag_rotation.rs

use std::cell::RefCell; // Provides interior mutability so several browser callbacks can update the same mouse-drag state.

use std::rc::Rc; // Lets the browser callbacks and renderer share ownership of the mouse-drag state.

use wasm_bindgen::closure::Closure; // Wraps Rust mouse-event functions so the browser can call them.

use wasm_bindgen::JsCast; // Converts Rust closures into the JavaScript callback type expected by browser event listeners.

pub(super) struct MouseDragRotation { // Stores the browser interaction state that controls the development cube orientation.

  yaw: f32, // Stores the current left-right rotation angle.

  pitch: f32, // Stores the current up-down rotation angle.

  dragging: bool, // Records whether the mouse button is currently being held down for cube rotation.

  last_x: i32, // Stores the previous horizontal mouse position in browser pixels.

  last_y: i32, // Stores the previous vertical mouse position in browser pixels.

} // Finishes the mouse-drag rotation state.

impl MouseDragRotation { // Defines the small public interface needed by the development-cube renderer.

  pub(super) fn angles(&self) -> [f32; 2] { // Returns only the two rotation values that the renderer needs.

    [ // Starts the two-angle result.

      self.yaw, // Returns the current horizontal rotation angle.

      self.pitch, // Returns the current vertical rotation angle.

    ] // Finishes the two-angle result.

  } // Finishes reading the current cube angles.

} // Finishes the mouse-drag rotation implementation.

pub(super) fn attach_mouse_drag_rotation( // Registers the browser mouse listeners and returns their shared rotation state.

  canvas: &web_sys::HtmlCanvasElement, // Borrows the development canvas on which mouse-down and mouse-move events are observed.

) -> Rc<RefCell<MouseDragRotation>> { // Returns shared state so the render loop can read the current yaw and pitch.

  let drag_state = Rc::new(RefCell::new( // Creates shared mutable mouse state for the event callbacks and renderer.

    MouseDragRotation { // Starts the initial development-cube orientation.

      yaw: 0.65, // Starts with the existing useful left-right viewing angle.

      pitch: 0.45, // Starts with the existing useful vertical tilt.

      dragging: false, // Starts with rotation inactive until the mouse button is pressed.

      last_x: 0, // Initializes the previous horizontal mouse position.

      last_y: 0, // Initializes the previous vertical mouse position.

    }, // Finishes the initial mouse-drag state.

  )); // Finishes creating the shared mouse-drag state.

  let drag_state_for_down = drag_state.clone(); // Gives the mouse-down callback access to the shared drag state.

  let mouse_down = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Starts dragging when the user presses the mouse button over the canvas.

    let mut state = drag_state_for_down.borrow_mut(); // Borrows the shared state so the callback can update it.

    state.dragging = true; // Marks the cube as actively being dragged.

    state.last_x = event.client_x(); // Remembers the horizontal position where this drag event begins.

    state.last_y = event.client_y(); // Remembers the vertical position where this drag event begins.

  }); // Finishes the mouse-down callback.

  canvas.add_event_listener_with_callback( // Registers the mouse-down callback on the development canvas.

    "mousedown", // Runs when the mouse button is pressed over the canvas.

    mouse_down.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mousedown listener"); // Stops clearly if the browser cannot register the listener.

  mouse_down.forget(); // Keeps the callback alive for the lifetime of the browser page.

  let drag_state_for_move = drag_state.clone(); // Gives the mouse-move callback access to the shared drag state.

  let mouse_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| { // Updates cube rotation while the mouse is dragged.

    let mut state = drag_state_for_move.borrow_mut(); // Borrows the shared drag state so the angles can be changed.

    if state.dragging { // Changes orientation only while the mouse button is being held.

      let delta_x = (event.client_x() - state.last_x) as f32; // Measures horizontal mouse movement since the previous event.

      let delta_y = (event.client_y() - state.last_y) as f32; // Measures vertical mouse movement since the previous event.

      state.yaw += delta_x * 0.01; // Converts horizontal movement into left-right cube rotation.

      state.pitch = (state.pitch + delta_y * 0.01).clamp(-1.4, 1.4); // Converts vertical movement into tilt while preventing a complete flip.

      state.last_x = event.client_x(); // Saves the current horizontal position for the next movement event.

      state.last_y = event.client_y(); // Saves the current vertical position for the next movement event.

    } // Finishes the active-drag check.

  }); // Finishes the mouse-move callback.

  canvas.add_event_listener_with_callback( // Registers mouse movement directly on the development canvas.

    "mousemove", // Runs whenever the pointer moves across the canvas.

    mouse_move.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mousemove listener"); // Stops clearly if the browser cannot register the listener.

  mouse_move.forget(); // Keeps the callback alive for the lifetime of the browser page.

  let drag_state_for_up = drag_state.clone(); // Gives the mouse-up callback access to the shared dragging flag.

  let mouse_up = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| { // Stops dragging when the mouse button is released.

    drag_state_for_up.borrow_mut().dragging = false; // Prevents further mouse movement from changing the cube orientation.

  }); // Finishes the mouse-up callback.

  web_sys::window().expect("Could not get browser window for cube mouseup listener").add_event_listener_with_callback( // Registers mouse-up on the window so releasing outside the canvas still ends the drag.

    "mouseup", // Runs whenever the pressed mouse button is released.

    mouse_up.as_ref().unchecked_ref(), // Converts the Rust closure into the callback type expected by the browser.

  ).expect("Could not register cube mouseup listener"); // Stops clearly if the browser cannot register the listener.

  mouse_up.forget(); // Keeps the callback alive for the lifetime of the browser page.

  drag_state // Returns the shared rotation state so the render loop can read its current angles.

} // Finishes registering development-cube mouse rotation.