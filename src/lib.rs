// Import useful tools from the wasm-bindgen library.
// wasm-bindgen lets Rust communicate with JavaScript/browser APIs.
use wasm_bindgen::prelude::*;


// This attribute tells wasm-bindgen:
// "Run the function below automatically when the WebAssembly module starts."
#[wasm_bindgen(start)]

// Define a public function named "start".
// Result<(), JsValue> means:
// - Ok(()) if everything succeeds
// - Err(JsValue) if a browser/JavaScript-related error occurs
pub fn start() -> Result<(), JsValue> {

    // Install a better panic handler.
    // If our Rust program crashes later, the browser console will show
    // a much more useful Rust error message.
    console_error_panic_hook::set_once();


    // Ask the browser for its Window object.
    // The Window represents the browser window/tab containing our webpage.
    let window = web_sys::window()

        // If there is somehow no browser Window, stop the program
        // and display this error message.
        .expect("Could not get browser window");


    // Ask the Window object for the HTML Document.
    // The Document represents our index.html page.
    let document = window

        // Get the document associated with this browser window.
        .document()

        // Stop with an error message if no document exists.
        .expect("Could not get document");


    // Search the HTML document for an element whose id is "status".
    // Later, index.html will contain something like:
    // <pre id="status">...</pre>
    let status = document

        // Perform the actual search for id="status".
        .get_element_by_id("status")

        // Stop with an error if that HTML element cannot be found.
        .expect("Could not find element with id=status");


    // Replace the text inside the HTML element we found.
    // If this appears in the browser, we know Rust/WASM started successfully.
    status.set_text_content(Some("Rust/WASM loaded successfully."));


    // Tell Rust that the function completed successfully.
    Ok(())

// Close the start function.
}