// src/runtime_diagnostics_overlay.rs

use wasm_bindgen::JsValue;

use crate::runtime_diagnostics::RuntimeDiagnosticsSnapshot;


pub(crate) struct RuntimeDiagnosticsOverlay {

  element: web_sys::Element,

  physics_dt: f64,

  chebyshev_order: usize,

  spectral_scale: f64,

  side_length: usize,

  x_line_count: usize,

  x_line_length: usize,

}


impl RuntimeDiagnosticsOverlay {

  pub(crate) fn new(

    physics_dt: f64,

    chebyshev_order: usize,

    spectral_scale: f64,

    side_length: usize,

    x_line_count: usize,

    x_line_length: usize,

  ) -> Result<Self, JsValue> {

    let window = web_sys::window()

      .ok_or_else(

        || JsValue::from_str(

          "Could not get browser window for runtime diagnostics.",

        ),

      )?;


    let document = window.document()

      .ok_or_else(

        || JsValue::from_str(

          "Could not get document for runtime diagnostics.",

        ),

      )?;


    let controls = document.get_element_by_id(

      "simulator-controls",

    )

    .ok_or_else(

      || JsValue::from_str(

        "Could not find simulator-controls for runtime diagnostics.",

      ),

    )?;


    let element = document.create_element(

      "pre",

    )?;


    element.set_attribute(

      "style",

      concat!(

        "margin: 0;",

        "padding: 10px 12px;",

        "background: #f4f4f4;",

        "border: 1px solid #888;",

        "border-radius: 6px;",

        "font-family: monospace;",

        "font-size: 12px;",

        "line-height: 1.4;",

        "color: #111;",

        "pointer-events: none;",

        "white-space: pre;",

      ),

    )?;


    controls.append_child(

      &element,

    )?;


    let overlay = Self {

      element,

      physics_dt,

      chebyshev_order,

      spectral_scale,

      side_length,

      x_line_count,

      x_line_length,

    };


    overlay.show_initial_state();


    Ok(
      overlay,
    )

  }


  pub(crate) fn update(

    &self,

    snapshot: &RuntimeDiagnosticsSnapshot,

  ) {

    let text = format!(

      concat!(

        "FPS:              {:>6.1}\n",

        "Frame time:       {:>6.1} ms\n",

        "Physics steps:    {:>6}\n",

        "Dropped total:    {:>6}\n",

        "Simulation time:  {:>6.2} s\n",

        "\n",

        "Physics dt:       {:>6.3} s\n",

        "Chebyshev order:  {:>6}\n",

        "Spectral scale:   {:>6.3}\n",

        "Grid:             {}³\n",

        "X-lines:          {} × {}",

      ),

      snapshot.frames_per_second,

      snapshot.frame_time_ms,

      snapshot.physics_steps_this_frame,

      snapshot.total_dropped_steps,

      snapshot.simulation_time,

      self.physics_dt,

      self.chebyshev_order,

      self.spectral_scale,

      self.side_length,

      self.x_line_count,

      self.x_line_length,

    );


    self.element.set_text_content(

      Some(
        &text,
      ),

    );

  }


  fn show_initial_state(

    &self,

  ) {

    let text = format!(

      concat!(

        "FPS:                 --\n",

        "Frame time:          -- ms\n",

        "Physics steps:        0\n",

        "Dropped total:        0\n",

        "Simulation time:   0.00 s\n",

        "\n",

        "Physics dt:       {:>6.3} s\n",

        "Chebyshev order:  {:>6}\n",

        "Spectral scale:   {:>6.3}\n",

        "Grid:             {}³\n",

        "X-lines:          {} × {}",

      ),

      self.physics_dt,

      self.chebyshev_order,

      self.spectral_scale,

      self.side_length,

      self.x_line_count,

      self.x_line_length,

    );


    self.element.set_text_content(

      Some(
        &text,
      ),

    );

  }

}