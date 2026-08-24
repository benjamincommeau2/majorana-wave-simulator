use wasm_bindgen::prelude::*; // Imports the common wasm-bindgen tools we need to connect Rust with WebAssembly.

#[wasm_bindgen(start)] // Tells wasm-bindgen to run the next function automatically when the WebAssembly module starts.

pub fn start() -> Result<(), JsValue> { // Defines the startup function and says it can either succeed or return a JavaScript-compatible error.
  console_error_panic_hook::set_once(); // Installs a panic hook so Rust errors are easier to read in the browser console.
  let window = web_sys::window().expect("Could not get browser window"); // Gets the browser's Window object and stops with this message if it is unavailable.
  let document = window.document().expect("Could not get document"); // Gets the HTML document loaded inside the browser window and stops if it is unavailable.
  let status = document.get_element_by_id("status").expect("Could not find element with id=status"); // Finds the HTML element whose id is "status" and stops if that element does not exist.
  status.set_text_content(Some("Rust/WASM loaded successfully.")); // Replaces the text inside the HTML status element with a success message.
  Ok(()) // Tells Rust that the startup function finished successfully and did not return an error.
  } // Closes the body of the start function.