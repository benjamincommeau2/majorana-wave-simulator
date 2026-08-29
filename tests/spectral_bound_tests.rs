use majorana_wave_simulator::physics::spectral_bound::conservative_dirac_spectral_scale_1d;

use majorana_wave_simulator::physics::spectral_bound::maximum_active_momentum_1d;

fn assert_approximately_equal(

  actual: f32,

  expected: f32,

) {

  let tolerance = 1.0e-5_f32;

  let difference =

    (actual - expected).abs();

  assert!(

    difference < tolerance,

    "actual = {actual}, expected = {expected}, difference = {difference}",

  );

}

#[test] // Verifies that an odd lattice includes its largest positive and negative spectral modes in the first derivative.

fn odd_grid_maximum_active_momentum_uses_largest_dft_mode() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let actual = maximum_active_momentum_1d(

    point_count,

    lattice_spacing,

  );

  let expected =

    4.0

    * std::f32::consts::PI

    / 5.0;

  assert_approximately_equal(

    actual,

    expected,

  );

}

#[test] // Verifies that the even-grid Nyquist slot is excluded because the project's first spectral derivative sets that slot to zero.

fn even_grid_maximum_active_momentum_excludes_nyquist() {

  let point_count = 4;

  let lattice_spacing = 1.0_f32;

  let actual = maximum_active_momentum_1d(

    point_count,

    lattice_spacing,

  );

  let expected =

    std::f32::consts::PI

    / 2.0;

  assert_approximately_equal(

    actual,

    expected,

  );

}

#[test] // Verifies the conservative triangle-inequality bound for any allowed spatial mass profile.

fn dirac_spectral_scale_adds_maximum_mass_magnitude() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let maximum_mass_magnitude = 3.0_f32;

  let actual = conservative_dirac_spectral_scale_1d(

    point_count,

    lattice_spacing,

    maximum_mass_magnitude,

  );

  let expected =

    4.0

    * std::f32::consts::PI

    / 5.0

    + maximum_mass_magnitude;

  assert_approximately_equal(

    actual,

    expected,

  );

}