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

#[test] // Verifies piecewise-static evolution: moving the mass boundary changes the next Hamiltonian without reinitializing the quantum state.

fn moved_mass_boundary_uses_previous_propagated_state() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let left_mass = 0.0_f32;

  let right_mass = 3.0_f32;

  let spectral_scale = 6.0_f32;

  let physics_dt = 0.1_f64;

  let max_order = 12;

  let initial_state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let initial_field = vec![

    initial_state;

    point_count

  ];

  let mass_profile_a = create_mass_step_profile_1d( // Starts with the mass boundary at lattice index 2.

    point_count,

    2,

    left_mass,

    right_mass,

  );

  let mass_profile_b = create_mass_step_profile_1d( // Represents the player moving the same boundary one lattice site to the right.

    point_count,

    3,

    left_mass,

    right_mass,

  );

  let coefficients = precompute_chebyshev_coefficients( // Computes one coefficient set that is reused for both frozen propagation intervals.

    spectral_scale as f64,

    physics_dt,

    max_order,

  );

  let state_after_a = direct_real_chebyshev_propagate_with_mass_profile_1d( // Evolves the initial quantum state under the first frozen Hamiltonian.

    &initial_field,

    lattice_spacing,

    &mass_profile_a,

    spectral_scale,

    &coefficients,

  );

  let state_after_b = direct_real_chebyshev_propagate_with_mass_profile_1d( // Continues from the already-evolved state after changing only the mass boundary.

    &state_after_a,

    lattice_spacing,

    &mass_profile_b,

    spectral_scale,

    &coefficients,

  );

  let incorrectly_reset_state_after_b = direct_real_chebyshev_propagate_with_mass_profile_1d( // Models the incorrect behavior of restarting from the original state when the boundary moves.

    &initial_field,

    lattice_spacing,

    &mass_profile_b,

    spectral_scale,

    &coefficients,

  );

  let tolerance = 1.0e-4_f32;

  let mut first_interval_changed_state = false;

  let mut continued_evolution_differs_from_reset = false;

  for spatial_index in 0..point_count {

    for component_index in 0..4 {

      let first_interval_difference =

        (

          state_after_a[spatial_index][component_index]

          - initial_field[spatial_index][component_index]

        )

        .abs();

      if first_interval_difference > tolerance {

        first_interval_changed_state = true;

      }

      let reset_difference =

        (

          state_after_b[spatial_index][component_index]

          - incorrectly_reset_state_after_b[spatial_index][component_index]

        )

        .abs();

      if reset_difference > tolerance {

        continued_evolution_differs_from_reset = true;

      }

    }

  }

  assert!(

    first_interval_changed_state,

    "The first frozen Hamiltonian interval should evolve the initial quantum state.",

  );

  assert!(

    continued_evolution_differs_from_reset,

    "The second interval must continue from the previously evolved state rather than restart from the initial state.",

  );

}