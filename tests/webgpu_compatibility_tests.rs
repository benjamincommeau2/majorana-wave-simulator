use majorana_wave_simulator::webgpu_compatibility::should_simulate_webgpu_unavailable;


#[test]

fn normal_query_does_not_simulate_webgpu_failure() {

  assert!(

    !should_simulate_webgpu_unavailable(

      "",

    ),

  );

}


#[test]

fn explicit_development_parameter_simulates_webgpu_failure() {

  assert!(

    should_simulate_webgpu_unavailable(

      "?simulate_webgpu_unavailable=1",

    ),

  );

}


#[test]

fn development_parameter_can_coexist_with_other_query_parameters() {

  assert!(

    should_simulate_webgpu_unavailable(

      "?foo=bar&simulate_webgpu_unavailable=1&answer=42",

    ),

  );

}


#[test]

fn zero_does_not_enable_simulated_failure() {

  assert!(

    !should_simulate_webgpu_unavailable(

      "?simulate_webgpu_unavailable=0",

    ),

  );

}