use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window()
        .expect("Could not get browser window");

    let document = window
        .document()
        .expect("Could not get document");

    let status = document
        .get_element_by_id("status")
        .expect("Could not find element with id=status");

    status.set_text_content(Some("Rust/WASM loaded successfully."));

    Ok(())
}