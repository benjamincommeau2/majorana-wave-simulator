use wasm_bindgen::prelude::*; // Imports the common wasm-bindgen tools we need to connect Rust with WebAssembly.

use wasm_bindgen_futures::spawn_local; // Imports the helper that lets browser-based Rust start asynchronous work without blocking the webpage.

#[wasm_bindgen(start)] // Tells wasm-bindgen to run the next function automatically when the WebAssembly module starts.

pub fn start() -> Result<(), JsValue> { // Defines the startup function and says it can either succeed or return a JavaScript-compatible error.
  
  console_error_panic_hook::set_once(); // Installs a panic hook so Rust errors are easier to read in the browser console.
  
  let window = web_sys::window().expect("Could not get browser window"); // Gets the browser's Window object and stops with this message if it is unavailable.
  
  let document = window.document().expect("Could not get document"); // Gets the HTML document loaded inside the browser window and stops if it is unavailable.
  
  let status = document.get_element_by_id("status").expect("Could not find element with id=status"); // Finds the HTML element whose id is "status" and stops if that element does not exist.

  status.set_text_content(Some("Rust/WASM loaded successfully.")); // Replaces the text inside the HTML status element with a success message.
  spawn_local(async move { // Starts an asynchronous Rust task where our WebGPU initialization will run.

    let instance = wgpu::Instance::default(); // Creates the main wgpu entry point used to discover and connect to available GPU adapters.

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.expect("Could not find a compatible GPU adapter"); // Asks wgpu for a GPU adapter, waits for the browser to return one, and stops with an error if none is available. In Rust, the ampersand (&) is primarily used to create or specify references, which allows you to borrow data without taking ownership of it.

    status.set_text_content(Some("WebGPU adapter acquired successfully.")); // Changes the webpage status only after Rust successfully receives a compatible GPU adapter.

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.expect("Could not create WebGPU device"); // Requests a logical GPU device and command queue from the adapter, waits for them to be created, and stops if the request fails.

    status.set_text_content(Some("WebGPU device and queue created successfully.")); // Updates the page only after Rust successfully creates the GPU device and command queue.
    
  }); // Closes the async block and finishes the spawn_local function call.

  Ok(()) // Tells Rust that the outer start function finished successfully.

} // Closes the outer start function.