use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_basis_1d; // Imports the tested direct CPU real Chebyshev-basis constructor.

use majorana_wave_simulator::physics::j_fourier_transform::apply_j_rotation; // Imports the tested J rotation used to construct an analytic Fourier mode.

fn assert_state_approximately_equal( // Compares two real Majorana states while allowing small f32 Fourier error.

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

#[test] // Verifies the initialization Phi_1 = (K / a) Psi against the analytic one-dimensional Dirac generator.

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

    let generator_at_position = [ // Evaluates K Psi = -alpha_x partial_x Psi + m(-i beta) Psi directly in components.

      -wave_number * component_d - mass * component_c,

      wave_number * component_c + mass * component_d,

      -wave_number * component_b + mass * component_a,

      wave_number * component_a - mass * component_b,

    ];

    let expected_state = [ // Uses Phi_1 = (K / a) Psi with the test scale a = E.

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

#[test] // Verifies the real Chebyshev recurrence Phi_2 = 2(K/a)Phi_1 + Phi_0 on a known Dirac eigenfrequency subspace.

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