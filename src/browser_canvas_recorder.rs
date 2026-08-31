use wasm_bindgen::closure::Closure;

use wasm_bindgen::JsCast;

use wasm_bindgen::JsValue;


const RECORDING_WIDTH: u32 =
  640;


const RECORDING_HEIGHT: u32 =
  640;


const RECORDING_FRAMES_PER_SECOND: f64 =
  30.0;


const RECORDING_DURATION_MILLISECONDS: i32 =
  20_000;


const VIDEO_BITS_PER_SECOND: u32 =
  3_000_000;


const DOWNLOAD_FILE_NAME: &str =
  "majorana-webgpu-linkedin-640x640-20s.webm";


const IDLE_BUTTON_TEXT: &str =
  "Record 20s LinkedIn clip";


pub fn attach_canvas_recorder(

  canvas: &web_sys::HtmlCanvasElement,

) -> Result<(), JsValue> {

  let window = web_sys::window()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get browser window for canvas recorder.",

      ),

    )?;


  let document = window.document()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get document for canvas recorder.",

      ),

    )?;


  let body = document.body()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get document body for canvas recorder.",

      ),

    )?;


  let recorder_controls = document.create_element(

    "div",

  )?;


  recorder_controls.set_attribute(

    "style",

    concat!(

      "position: fixed;",

      "top: 12px;",

      "right: 12px;",

      "z-index: 1000;",

      "display: flex;",

      "gap: 10px;",

      "align-items: center;",

      "padding: 8px 10px;",

      "background: rgba(255, 255, 255, 0.92);",

      "border: 1px solid #888;",

      "border-radius: 6px;",

      "font-family: monospace;",

      "font-size: 13px;",

    ),

  )?;


  let record_button = document.create_element(

    "button",

  )?;


  record_button.set_text_content(

    Some(

      IDLE_BUTTON_TEXT,

    ),

  );


  record_button.set_attribute(

    "type",

    "button",

  )?;


  let recording_status = document.create_element(

    "span",

  )?;


  recording_status.set_text_content(

    Some(

      "Ready",

    ),

  );


  recorder_controls.append_child(

    &record_button,

  )?;


  recorder_controls.append_child(

    &recording_status,

  )?;


  body.append_child(

    &recorder_controls,

  )?;


  let canvas_for_click =

    canvas.clone();


  let button_for_click =

    record_button.clone();


  let status_for_click =

    recording_status.clone();


  let click_callback = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(

    move |_event| {

      let recording_result = start_recording(

        &canvas_for_click,

        &button_for_click,

        &status_for_click,

      );


      if let Err(error) = recording_result {

        let _ = button_for_click.remove_attribute(

          "disabled",

        );


        button_for_click.set_text_content(

          Some(

            IDLE_BUTTON_TEXT,

          ),

        );


        let error_message = error

          .as_string()

          .unwrap_or_else(

            || format!(

              "{error:?}",

            ),

          );


        status_for_click.set_text_content(

          Some(

            &format!(

              "Recording failed: {error_message}",

            ),

          ),

        );

      }

    },

  );


  record_button.add_event_listener_with_callback(

    "click",

    click_callback

      .as_ref()

      .unchecked_ref(),

  )?;


  // This listener exists for the entire lifetime of the page,
  // so intentionally keep its Rust closure alive.

  click_callback.forget();


  Ok(())

}


fn start_recording(

  canvas: &web_sys::HtmlCanvasElement,

  record_button: &web_sys::Element,

  recording_status: &web_sys::Element,

) -> Result<(), JsValue> {

  verify_canvas_dimensions(

    canvas,

  )?;


  record_button.set_attribute(

    "disabled",

    "",

  )?;


  record_button.set_text_content(

    Some(

      "Recording...",

    ),

  );


  recording_status.set_text_content(

    Some(

      "20 seconds — drag the field now",

    ),

  );


  let media_stream = canvas.capture_stream_with_frame_request_rate(

    RECORDING_FRAMES_PER_SECOND,

  )?;


  let mime_type = choose_webm_mime_type()?;


  let recorder_options =

    web_sys::MediaRecorderOptions::new();


  recorder_options.set_mime_type(

    mime_type,

  );


  recorder_options.set_video_bits_per_second(

    VIDEO_BITS_PER_SECOND,

  );


  let media_recorder = web_sys::MediaRecorder::new_with_media_stream_and_media_recorder_options(

    &media_stream,

    &recorder_options,

  )?;


  attach_recording_data_handler(

    &media_recorder,

    record_button,

    recording_status,

  );


  media_recorder.start()?;


  schedule_automatic_stop(

    media_recorder,

    record_button.clone(),

    recording_status.clone(),

  )?;


  Ok(())

}


fn verify_canvas_dimensions(

  canvas: &web_sys::HtmlCanvasElement,

) -> Result<(), JsValue> {

  if canvas.width() != RECORDING_WIDTH

    || canvas.height() != RECORDING_HEIGHT

  {

    return Err(

      JsValue::from_str(

        &format!(

          "LinkedIn recorder expected a {}x{} canvas, but the canvas is {}x{}.",

          RECORDING_WIDTH,

          RECORDING_HEIGHT,

          canvas.width(),

          canvas.height(),

        ),

      ),

    );

  }


  Ok(())

}


fn choose_webm_mime_type() -> Result<&'static str, JsValue> {

  const CANDIDATES: [&str; 3] = [

    "video/webm;codecs=vp9",

    "video/webm;codecs=vp8",

    "video/webm",

  ];


  for candidate in CANDIDATES {

    if web_sys::MediaRecorder::is_type_supported(

      candidate,

    ) {

      return Ok(

        candidate,

      );

    }

  }


  Err(

    JsValue::from_str(

      "This browser does not report support for WebM MediaRecorder output.",

    ),

  )

}


fn attach_recording_data_handler(

  media_recorder: &web_sys::MediaRecorder,

  record_button: &web_sys::Element,

  recording_status: &web_sys::Element,

) {

  let button_for_data =

    record_button.clone();


  let status_for_data =

    recording_status.clone();


  // We start MediaRecorder without a timeslice.
  //
  // Therefore the completed recording is delivered when
  // MediaRecorder::stop() produces the final dataavailable event.

  let data_callback = Closure::once_into_js(

    move |event: web_sys::BlobEvent| {

      let recording_result = save_recording_event(

        event,

      );


      match recording_result {

        Ok(file_size_megabytes) => {

          status_for_data.set_text_content(

            Some(

              &format!(

                "Saved {:.1} MB",

                file_size_megabytes,

              ),

            ),

          );

        }


        Err(error) => {

          let error_message = error

            .as_string()

            .unwrap_or_else(

              || format!(

                "{error:?}",

              ),

            );


          status_for_data.set_text_content(

            Some(

              &format!(

                "Save failed: {error_message}",

              ),

            ),

          );

        }

      }


      let _ = button_for_data.remove_attribute(

        "disabled",

      );


      button_for_data.set_text_content(

        Some(

          IDLE_BUTTON_TEXT,

        ),

      );

    },

  );


  media_recorder.set_ondataavailable(

    Some(

      data_callback.unchecked_ref(),

    ),

  );

}


fn schedule_automatic_stop(

  media_recorder: web_sys::MediaRecorder,

  record_button: web_sys::Element,

  recording_status: web_sys::Element,

) -> Result<(), JsValue> {

  let window = web_sys::window()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get browser window for recorder timer.",

      ),

    )?;


  let stop_callback = Closure::once_into_js(

    move || {

      if let Err(error) = media_recorder.stop() {

        let _ = record_button.remove_attribute(

          "disabled",

        );


        record_button.set_text_content(

          Some(

            IDLE_BUTTON_TEXT,

          ),

        );


        let error_message = error

          .as_string()

          .unwrap_or_else(

            || format!(

              "{error:?}",

            ),

          );


        recording_status.set_text_content(

          Some(

            &format!(

              "Could not stop recording: {error_message}",

            ),

          ),

        );

      }

    },

  );


  window.set_timeout_with_callback_and_timeout_and_arguments_0(

    stop_callback.unchecked_ref(),

    RECORDING_DURATION_MILLISECONDS,

  )?;


  Ok(())

}


fn save_recording_event(

  event: web_sys::BlobEvent,

) -> Result<f64, JsValue> {

  let recording_blob = event.data()

    .ok_or_else(

      || JsValue::from_str(

        "MediaRecorder returned no video blob.",

      ),

    )?;


  if recording_blob.size() <= 0.0 {

    return Err(

      JsValue::from_str(

        "MediaRecorder returned an empty video file.",

      ),

    );

  }


  let file_size_megabytes =

    recording_blob.size()

    / 1_000_000.0;


  download_blob(

    &recording_blob,

  )?;


  Ok(

    file_size_megabytes,

  )

}


fn download_blob(

  recording_blob: &web_sys::Blob,

) -> Result<(), JsValue> {

  let window = web_sys::window()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get browser window while saving recording.",

      ),

    )?;


  let document = window.document()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get document while saving recording.",

      ),

    )?;


  let body = document.body()

    .ok_or_else(

      || JsValue::from_str(

        "Could not get document body while saving recording.",

      ),

    )?;


  let object_url = web_sys::Url::create_object_url_with_blob(

    recording_blob,

  )?;


  let download_link = document.create_element(

    "a",

  )?;


  download_link.set_attribute(

    "href",

    &object_url,

  )?;


  download_link.set_attribute(

    "download",

    DOWNLOAD_FILE_NAME,

  )?;


  download_link.set_attribute(

    "style",

    "display: none;",

  )?;


  body.append_child(

    &download_link,

  )?;


  let download_html_element = download_link

    .dyn_ref::<web_sys::HtmlElement>()

    .ok_or_else(

      || JsValue::from_str(

        "Could not convert recording download link to HtmlElement.",

      ),

    )?;


  download_html_element.click();


  download_link.remove();


  // Keep the object URL alive briefly so the browser has time
  // to begin the download before its backing Blob URL is revoked.

  let object_url_for_cleanup =

    object_url.clone();


  let cleanup_callback = Closure::once_into_js(

    move || {

      let _ = web_sys::Url::revoke_object_url(

        &object_url_for_cleanup,

      );

    },

  );


  let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(

    cleanup_callback.unchecked_ref(),

    1_000,

  );


  Ok(())

}