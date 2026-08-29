use majorana_wave_simulator::physics::dirac_generator::apply_dirac_mass_generator; // Imports the tested local real Majorana mass generator.

use majorana_wave_simulator::physics::dirac_generator::direct_dirac_generator_1d; // Imports the complete one-dimensional real Majorana time generator that does not exist yet.

use majorana_wave_simulator::physics::dirac_generator::direct_dirac_kinetic_generator_x_1d; // Imports the tested one-dimensional kinetic generator K_x = -alpha_x partial_x.

use majorana_wave_simulator::physics::j_fourier_transform::apply_j_rotation; // Imports the tested real J rotation used to construct analytic Fourier modes.

fn assert_state_approximately_equal( // Compares two Majorana states while allowing small f32 numerical error.

  actual: &[f32; 4],

  expected: &[f32; 4],

) {

  let tolerance = 1.0e-4_f32;

  for component_index in 0..4 {

    let difference =

      (actual[component_index] - expected[component_index]).abs();

    assert!(

      difference < tolerance,

      "component {component_index} differed: actual = {}, expected = {}",

      actual[component_index],

      expected[component_index],

    );

  }

}

#[test] // Verifies K_x = -alpha_x partial_x against an analytic positive-frequency Majorana mode.

fn dirac_kinetic_generator_x_matches_analytic_positive_frequency_mode() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let domain_length =

    point_count as f32

    * lattice_spacing;

  let wave_number =

    std::f32::consts::TAU

    / domain_length;

  let base_state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let mut field = Vec::with_capacity(

    point_count,

  );

  let mut expected_generator = Vec::with_capacity(

    point_count,

  );

  for spatial_index in 0..point_count {

    let position =

      spatial_index as f32

      * lattice_spacing;

    let angle =

      wave_number

      * position;

    let state_at_position = apply_j_rotation( // Constructs Psi(x) = exp(J k x) Psi_0.

      &base_state,

      angle,

    );

    let [

      component_a,

      component_b,

      component_c,

      component_d,

    ] = state_at_position;

    let expected_at_position = [ // Uses -k alpha_x J(a,b,c,d) = (-kd,+kc,-kb,+ka).

      -wave_number * component_d,

      wave_number * component_c,

      -wave_number * component_b,

      wave_number * component_a,

    ];

    field.push(

      state_at_position,

    );

    expected_generator.push(

      expected_at_position,

    );

  }

  let numerical_generator = direct_dirac_kinetic_generator_x_1d(

    &field,

    lattice_spacing,

  );

  assert_eq!(

    numerical_generator.len(),

    point_count,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &numerical_generator[spatial_index],

      &expected_generator[spatial_index],

    );

  }

}

#[test] // Verifies that the real mass term is K_m = m(-i beta) with the locked Majorana-basis component mapping.

fn dirac_mass_generator_uses_expected_real_majorana_mapping() {

  let state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let mass = 2.0_f32;

  let generated_state = apply_dirac_mass_generator(

    &state,

    mass,

  );

  let expected = [ // Uses m(-i beta)(a,b,c,d) = m(-c,d,a,-b).

    -6.0,

    8.0,

    2.0,

    -4.0,

  ];

  assert_state_approximately_equal(

    &generated_state,

    &expected,

  );

}

#[test] // Verifies the complete one-dimensional real Majorana time generator against an analytic Fourier mode.

fn complete_dirac_generator_1d_combines_kinetic_and_mass_terms() {

  let point_count = 5; // Uses an odd periodic grid so the Nyquist convention cannot influence the result.

  let lattice_spacing = 1.0_f32;

  let domain_length =

    point_count as f32

    * lattice_spacing;

  let wave_number =

    std::f32::consts::TAU

    / domain_length;

  let mass = 0.75_f32; // Uses a nonzero noninteger mass so both kinetic and mass contributions are visible.

  let base_state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let mut field = Vec::with_capacity(

    point_count,

  );

  let mut expected_generator = Vec::with_capacity(

    point_count,

  );

  for spatial_index in 0..point_count {

    let position =

      spatial_index as f32

      * lattice_spacing;

    let angle =

      wave_number

      * position;

    let state_at_position = apply_j_rotation( // Constructs Psi(x) = exp(J k x) Psi_0.

      &base_state,

      angle,

    );

    let [

      component_a,

      component_b,

      component_c,

      component_d,

    ] = state_at_position;

    let expected_at_position = [ // Evaluates K = -alpha_x partial_x + m(-i beta) directly in components.

      -wave_number * component_d - mass * component_c,

      wave_number * component_c + mass * component_d,

      -wave_number * component_b + mass * component_a,

      wave_number * component_a - mass * component_b,

    ];

    field.push(

      state_at_position,

    );

    expected_generator.push(

      expected_at_position,

    );

  }

  let numerical_generator = direct_dirac_generator_1d(

    &field,

    lattice_spacing,

    mass,

  );

  assert_eq!(

    numerical_generator.len(),

    point_count,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &numerical_generator[spatial_index],

      &expected_generator[spatial_index],

    );

  }

}

#[test] // Verifies that applying the complete Dirac generator twice reproduces the relativistic dispersion relation.

fn dirac_generator_squared_matches_relativistic_dispersion() {

  let point_count = 5; // Uses an odd grid so the Nyquist convention cannot affect the analytic Fourier mode.

  let lattice_spacing = 1.0_f32;

  let domain_length =

    point_count as f32

    * lattice_spacing;

  let wave_number =

    std::f32::consts::TAU

    / domain_length;

  let mass = 0.75_f32;

  let base_state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let mut field = Vec::with_capacity(

    point_count,

  );

  for spatial_index in 0..point_count {

    let position =

      spatial_index as f32

      * lattice_spacing;

    let angle =

      wave_number

      * position;

    let state_at_position = apply_j_rotation(

      &base_state,

      angle,

    );

    field.push(

      state_at_position,

    );

  }

  let generator_once = direct_dirac_generator_1d(

    &field,

    lattice_spacing,

    mass,

  );

  let generator_twice = direct_dirac_generator_1d(

    &generator_once,

    lattice_spacing,

    mass,

  );

  let squared_frequency =

    wave_number * wave_number

    + mass * mass;

  for spatial_index in 0..point_count {

    let expected = [ // Uses K squared Psi = -(k squared + m squared) Psi.

      -squared_frequency * field[spatial_index][0],

      -squared_frequency * field[spatial_index][1],

      -squared_frequency * field[spatial_index][2],

      -squared_frequency * field[spatial_index][3],

    ];

    assert_state_approximately_equal(

      &generator_twice[spatial_index],

      &expected,

    );

  }

}