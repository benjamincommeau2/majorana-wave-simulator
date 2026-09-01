// src/mass_boundary_control.rs

use std::cell::Cell;

use std::rc::Rc;

use wasm_bindgen::closure::Closure;

use wasm_bindgen::JsCast;

use wasm_bindgen::JsValue;


pub(crate) struct MassBoundaryControl {

  boundary_index: Rc<Cell<usize>>,

}


impl MassBoundaryControl {

  pub(crate) fn new(

    maximum_boundary_index: usize,

    initial_boundary_index: usize,

  ) -> Result<Self, JsValue> {

    assert!(

      initial_boundary_index
        <= maximum_boundary_index,

      "Initial mass boundary must lie inside the x-grid.",

    );


    let window = web_sys::window()

      .ok_or_else(

        || JsValue::from_str(

          "Could not get browser window for mass-boundary control.",

        ),

      )?;


    let document = window.document()

      .ok_or_else(

        || JsValue::from_str(

          "Could not get document for mass-boundary control.",

        ),

      )?;


    let controls = document.get_element_by_id(

      "simulator-controls",

    )

    .ok_or_else(

      || JsValue::from_str(

        "Could not find simulator-controls for mass-boundary control.",

      ),

    )?;


    let container = document.create_element(

      "div",

    )?;


    container.set_attribute(

      "style",

      concat!(

        "margin: 0;",

        "padding: 10px 12px;",

        "width: fit-content;",

        "background: #f4f4f4;",

        "border: 1px solid #888;",

        "border-radius: 6px;",

        "font-family: monospace;",

        "font-size: 13px;",

      ),

    )?;


    let title = document.create_element(

      "div",

    )?;


    title.set_text_content(

      Some(

        "Interactive mass boundary",

      ),

    );


    title.set_attribute(

      "style",

      "font-weight: bold; margin-bottom: 8px;",

    )?;


    let value_label = document.create_element(

      "span",

    )?;


    value_label.set_text_content(

      Some(

        &format!(

          "x = {initial_boundary_index}",

        ),

      ),

    );


    let slider_element = document.create_element(

      "input",

    )?;


    let slider = slider_element

      .dyn_into::<web_sys::HtmlInputElement>()

      .map_err(

        |_| JsValue::from_str(

          "Could not create mass-boundary range input.",

        ),

      )?;


    slider.set_type(

      "range",

    );


    slider.set_min(

      "0",

    );


    slider.set_max(

      &maximum_boundary_index.to_string(),

    );


    slider.set_step(

      "1",

    );


    slider.set_value(

      &initial_boundary_index.to_string(),

    );


    slider.set_attribute(

      "style",

      "width: 320px; margin-right: 12px; vertical-align: middle;",

    )?;


    container.append_child(

      &title,

    )?;


    container.append_child(

      &slider,

    )?;


    container.append_child(

      &value_label,

    )?;


    controls.append_child(

      &container,

    )?;


    let boundary_index = Rc::new(

      Cell::new(

        initial_boundary_index,

      ),

    );


    let boundary_index_for_input =

      boundary_index.clone();


    let slider_for_input =

      slider.clone();


    let value_label_for_input =

      value_label.clone();


    let input_callback = Closure::<dyn FnMut(web_sys::Event)>::new(

      move |_event| {

        let requested_boundary = slider_for_input

          .value_as_number()

          .round()

          .clamp(

            0.0,

            maximum_boundary_index as f64,

          ) as usize;


        boundary_index_for_input.set(

          requested_boundary,

        );


        value_label_for_input.set_text_content(

          Some(

            &format!(

              "x = {requested_boundary}",

            ),

          ),

        );

      },

    );


    slider.add_event_listener_with_callback(

      "input",

      input_callback

        .as_ref()

        .unchecked_ref(),

    )?;


    input_callback.forget();


    Ok(

      Self {

        boundary_index,

      },

    )

  }


  pub(crate) fn boundary_index(

    &self,

  ) -> usize {

    self.boundary_index.get()

  }

}