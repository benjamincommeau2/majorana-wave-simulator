// src/webgpu_compatibility.rs

pub fn should_simulate_webgpu_unavailable(

  query_string: &str,

) -> bool {

  query_string

    .trim_start_matches(

      '?',

    )

    .split(

      '&',

    )

    .any(

      |parameter| {

        parameter
          == "simulate_webgpu_unavailable=1"

      },

    )

}