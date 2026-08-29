use majorana_wave_simulator::physics::dirac_generator::direct_dirac_generator_1d;

use majorana_wave_simulator::physics::dirac_generator::direct_dirac_generator_with_mass_profile_1d;

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d;

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

#[test] // Verifies that a spatially uniform mass field reproduces the already-tested scalar-mass generator.

fn uniform_mass_profile_matches_scalar_mass_generator() {

  let lattice_spacing = 1.0_f32;

  let mass = 0.75_f32;

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

  let scalar_mass_result = direct_dirac_generator_1d(

    &field,

    lattice_spacing,

    mass,

  );

  let spatial_mass_result = direct_dirac_generator_with_mass_profile_1d(

    &field,

    lattice_spacing,

    &mass_profile,

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

#[test] // Verifies that a piecewise mass step changes the local mass-generator contribution exactly at the chosen boundary.

fn mass_step_profile_changes_generator_at_boundary() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let boundary_index = 2;

  let left_mass = 0.0_f32;

  let right_mass = 3.0_f32;

  let state = [

    1.0,

    2.0,

    3.0,

    4.0,

  ];

  let field = vec![ // Uses a spatially constant field so its spectral derivative vanishes and only the mass term remains.

    state;

    point_count

  ];

  let mass_profile = create_mass_step_profile_1d(

    point_count,

    boundary_index,

    left_mass,

    right_mass,

  );

  let generated = direct_dirac_generator_with_mass_profile_1d(

    &field,

    lattice_spacing,

    &mass_profile,

  );

  let zero_state = [

    0.0,

    0.0,

    0.0,

    0.0,

  ];

  let right_mass_state = [ // Uses 3 B(1,2,3,4) = 3(-3,4,1,-2).

    -9.0,

    12.0,

    3.0,

    -6.0,

  ];

  assert_state_approximately_equal(

    &generated[0],

    &zero_state,

  );

  assert_state_approximately_equal(

    &generated[1],

    &zero_state,

  );

  assert_state_approximately_equal(

    &generated[2],

    &right_mass_state,

  );

  assert_state_approximately_equal(

    &generated[3],

    &right_mass_state,

  );

  assert_state_approximately_equal(

    &generated[4],

    &right_mass_state,

  );

}