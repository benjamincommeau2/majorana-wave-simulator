use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_basis_1d; // Imports the tested direct CPU real Chebyshev-basis constructor.

use majorana_wave_simulator::physics::chebyshev_propagator::precompute_chebyshev_coefficients; // Imports the simulation-setup coefficient constructor that does not exist yet.

use majorana_wave_simulator::physics::j_fourier_transform::apply_j_rotation; // Imports the tested J rotation used to construct an analytic Fourier mode.

fn assert_state_approximately_equal(

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

fn assert_f64_approximately_equal(

  actual: f64,

  expected: f64,

) {

  let tolerance = 1.0e-13_f64;

  let difference =

    (actual - expected).abs();

  assert!(

    difference < tolerance,

    "value differed: actual = {actual}, expected = {expected}, difference = {difference}",

  );

}

#[test]

fn first_real_chebyshev_basis_state_matches_scaled_dirac_generator() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let domain_length =

    point_count as f32

    * lattice_spacing;

  let wave_number =

    std::f32::consts::TAU

    / domain_length;

  let mass = 0.75_f32;

  let wave_energy =

    (

      wave_number * wave_number

      + mass * mass

    )

    .sqrt();

  let base_state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let mut field = Vec::with_capacity(

    point_count,

  );

  let mut expected_phi_one = Vec::with_capacity(

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

    let [

      component_a,

      component_b,

      component_c,

      component_d,

    ] = state_at_position;

    let generator_at_position = [

      -wave_number * component_d - mass * component_c,

      wave_number * component_c + mass * component_d,

      -wave_number * component_b + mass * component_a,

      wave_number * component_a - mass * component_b,

    ];

    let expected_state = [

      generator_at_position[0] / wave_energy,

      generator_at_position[1] / wave_energy,

      generator_at_position[2] / wave_energy,

      generator_at_position[3] / wave_energy,

    ];

    field.push(

      state_at_position,

    );

    expected_phi_one.push(

      expected_state,

    );

  }

  let basis_states = direct_real_chebyshev_basis_1d(

    &field,

    lattice_spacing,

    mass,

    wave_energy,

    1,

  );

  assert_eq!(

    basis_states.len(),

    2,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &basis_states[0][spatial_index],

      &field[spatial_index],

    );

    assert_state_approximately_equal(

      &basis_states[1][spatial_index],

      &expected_phi_one[spatial_index],

    );

  }

}

#[test]

fn second_real_chebyshev_basis_state_negates_single_energy_mode() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let domain_length =

    point_count as f32

    * lattice_spacing;

  let wave_number =

    std::f32::consts::TAU

    / domain_length;

  let mass = 0.75_f32;

  let wave_energy =

    (

      wave_number * wave_number

      + mass * mass

    )

    .sqrt();

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

  let basis_states = direct_real_chebyshev_basis_1d(

    &field,

    lattice_spacing,

    mass,

    wave_energy,

    2,

  );

  assert_eq!(

    basis_states.len(),

    3,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &basis_states[0][spatial_index],

      &field[spatial_index],

    );

    let expected_second_state = [

      -field[spatial_index][0],

      -field[spatial_index][1],

      -field[spatial_index][2],

      -field[spatial_index][3],

    ];

    assert_state_approximately_equal(

      &basis_states[2][spatial_index],

      &expected_second_state,

    );

  }

}

#[test] // Verifies our propagator coefficient convention while relying on libm for the Bessel-function calculation itself.

fn chebyshev_coefficients_use_expected_bessel_weights() {

  let spectral_scale = 2.0_f64;

  let physics_dt = 0.5_f64;

  let max_order = 3;

  let coefficients = precompute_chebyshev_coefficients(

    spectral_scale,

    physics_dt,

    max_order,

  );

  assert_eq!(

    coefficients.len(),

    max_order + 1,

  );

  let argument = // Gives z = a times delta t = 1, where standard Bessel values are easy to cross-check.

    spectral_scale

    * physics_dt;

  assert_f64_approximately_equal(

    coefficients[0],

    libm::jn(

      0,

      argument,

    ),

  );

  for order in 1..=max_order {

    assert_f64_approximately_equal(

      coefficients[order],

      2.0

        * libm::jn(

          order as i32,

          argument,

        ),

    );

  }

}