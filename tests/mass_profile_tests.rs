use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d; // Imports the one-dimensional piecewise mass-step constructor that does not exist yet.

#[test] // Verifies that the boundary index is the first lattice site belonging to the right-hand mass region.

fn mass_step_profile_uses_expected_left_and_right_regions() {

  let point_count = 5;

  let boundary_index = 2;

  let left_mass = 0.0_f32;

  let right_mass = 3.0_f32;

  let mass_profile = create_mass_step_profile_1d(

    point_count,

    boundary_index,

    left_mass,

    right_mass,

  );

  let expected = [

    0.0,

    0.0,

    3.0,

    3.0,

    3.0,

  ];

  assert_eq!(

    mass_profile,

    expected,

  );

}