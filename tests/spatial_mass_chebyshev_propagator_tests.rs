use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_basis_1d; // Imports the already-tested scalar-mass CPU Chebyshev basis reference.

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_basis_with_mass_profile_1d; // Imports the tested spatial-mass Chebyshev basis reference.

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_propagate_1d; // Imports the already-tested scalar-mass propagation reference.

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_propagate_with_mass_profile_1d; // Imports the spatial-mass propagator that does not exist yet.

use majorana_wave_simulator::physics::chebyshev_propagator::precompute_chebyshev_coefficients; // Reuses the setup-time Bessel coefficient calculation.

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d; // Constructs the piecewise spatial mass boundary used by the interactive architecture.

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

#[test] // Verifies that replacing one scalar mass with an equivalent uniform mass profile does not change any Chebyshev basis state.

fn uniform_mass_profile_matches_scalar_mass_chebyshev_basis() {

  let lattice_spacing = 1.0_f32;

  let mass = 0.75_f32;

  let spectral_scale = 4.0_f32;

  let max_order = 3;

  let field = [

    [1.0, 2.0, 3.0, 4.0],

    [5.0, 6.0, 7.0, 8.0],

    [9.0, 10.0, 11.0, 12.0],

    [13.0, 14.0, 15.0, 16.0],

    [17.0, 18.0, 19.0, 20.0],

  ];

  let mass_profile = vec![

    mass;

    field.len()

  ];

  let scalar_mass_basis = direct_real_chebyshev_basis_1d(

    &field,

    lattice_spacing,

    mass,

    spectral_scale,

    max_order,

  );

  let spatial_mass_basis = direct_real_chebyshev_basis_with_mass_profile_1d(

    &field,

    lattice_spacing,

    &mass_profile,

    spectral_scale,

    max_order,

  );

  assert_eq!(

    spatial_mass_basis.len(),

    scalar_mass_basis.len(),

  );

  for order in 0..=max_order {

    assert_eq!(

      spatial_mass_basis[order].len(),

      field.len(),

    );

    for spatial_index in 0..field.len() {

      assert_state_approximately_equal(

        &spatial_mass_basis[order][spatial_index],

        &scalar_mass_basis[order][spatial_index],

      );

    }

  }

}

#[test] // Verifies that Phi_1 responds to the actual piecewise mass boundary without changing the incoming quantum state.

fn mass_step_profile_sets_expected_first_chebyshev_basis_state() {

  let point_count = 5;

  let boundary_index = 2;

  let lattice_spacing = 1.0_f32;

  let left_mass = 0.0_f32;

  let right_mass = 3.0_f32;

  let spectral_scale = 6.0_f32;

  let state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let field = vec![

    state;

    point_count

  ];

  let mass_profile = create_mass_step_profile_1d(

    point_count,

    boundary_index,

    left_mass,

    right_mass,

  );

  let basis = direct_real_chebyshev_basis_with_mass_profile_1d(

    &field,

    lattice_spacing,

    &mass_profile,

    spectral_scale,

    1,

  );

  let zero_state = [

    0.0,

    0.0,

    0.0,

    0.0,

  ];

  let right_phi_one = [

    -1.5,

    2.0,

    0.5,

    -1.0,

  ];

  assert_eq!(

    basis.len(),

    2,

  );

  assert_state_approximately_equal(

    &basis[0][0],

    &state,

  );

  assert_state_approximately_equal(

    &basis[0][4],

    &state,

  );

  assert_state_approximately_equal(

    &basis[1][0],

    &zero_state,

  );

  assert_state_approximately_equal(

    &basis[1][1],

    &zero_state,

  );

  assert_state_approximately_equal(

    &basis[1][2],

    &right_phi_one,

  );

  assert_state_approximately_equal(

    &basis[1][3],

    &right_phi_one,

  );

  assert_state_approximately_equal(

    &basis[1][4],

    &right_phi_one,

  );

}

#[test] // Verifies that a uniform spatial mass profile reproduces the already-tested scalar-mass propagation result.

fn uniform_mass_profile_matches_scalar_mass_chebyshev_propagation() {

  let lattice_spacing = 1.0_f32;

  let mass = 0.75_f32;

  let spectral_scale = 4.0_f32;

  let physics_dt = 0.1_f64;

  let max_order = 12;

  let field = [

    [1.0, 2.0, 3.0, 4.0],

    [5.0, 6.0, 7.0, 8.0],

    [9.0, 10.0, 11.0, 12.0],

    [13.0, 14.0, 15.0, 16.0],

    [17.0, 18.0, 19.0, 20.0],

  ];

  let mass_profile = vec![

    mass;

    field.len()

  ];

  let coefficients = precompute_chebyshev_coefficients(

    spectral_scale as f64,

    physics_dt,

    max_order,

  );

  let scalar_mass_result = direct_real_chebyshev_propagate_1d(

    &field,

    lattice_spacing,

    mass,

    spectral_scale,

    &coefficients,

  );

  let spatial_mass_result = direct_real_chebyshev_propagate_with_mass_profile_1d(

    &field,

    lattice_spacing,

    &mass_profile,

    spectral_scale,

    &coefficients,

  );

  assert_eq!(

    spatial_mass_result.len(),

    scalar_mass_result.len(),

  );

  for spatial_index in 0..field.len() {

    assert_state_approximately_equal(

      &spatial_mass_result[spatial_index],

      &scalar_mass_result[spatial_index],

    );

  }

}