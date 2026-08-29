use majorana_wave_simulator::physics::momentum_grid::create_momentum_grid_1d; // Imports the production one-dimensional momentum-grid constructor.

fn assert_f32_approximately_equal( // Compares two floating-point momentum values while allowing small f32 rounding error.

  actual: f32,

  expected: f32,

) {

  let tolerance = 1.0e-5_f32; // Uses a small tolerance appropriate for analytically simple momentum values.

  let difference = (actual - expected).abs(); // Measures the absolute error between the actual and expected momentum.

  assert!( // Requires the momentum error to stay below the chosen tolerance.

    difference < tolerance,

    "momentum differed: actual = {actual}, expected = {expected}",

  );

}

#[test] // Verifies the signed DFT-order momentum convention on an odd-sized periodic lattice.

fn odd_length_momentum_grid_uses_expected_signed_dft_order() {

  let point_count = 5; // Uses an odd number of points so there is no Nyquist slot.

  let lattice_spacing = 1.0; // Uses unit lattice spacing so the expected values remain easy to inspect.

  let momentum_grid = create_momentum_grid_1d(

    point_count,

    lattice_spacing,

  );

  assert_eq!(

    momentum_grid.len(),

    point_count,

  );

  let momentum_step = std::f32::consts::TAU / 5.0; // Computes 2 pi divided by L = 5.

  let expected = [

    0.0,

    momentum_step,

    2.0 * momentum_step,

    -2.0 * momentum_step,

    -momentum_step,

  ];

  for momentum_index in 0..point_count {

    assert_f32_approximately_equal(

      momentum_grid[momentum_index],

      expected[momentum_index],

    );

  }

}

#[test] // Verifies the chosen unshifted DFT convention for an even grid, including the negative Nyquist momentum.

fn even_length_momentum_grid_places_negative_nyquist_at_halfway_index() {

  let point_count = 4; // Uses the smallest convenient even grid that contains positive, negative, and Nyquist modes.

  let lattice_spacing = 1.0; // Gives domain length L = 4 and momentum spacing pi / 2.

  let momentum_grid = create_momentum_grid_1d(

    point_count,

    lattice_spacing,

  );

  assert_eq!(

    momentum_grid.len(),

    point_count,

  );

  let momentum_step = std::f32::consts::FRAC_PI_2; // For N = 4 and delta x = 1, delta k = 2 pi / 4 = pi / 2.

  let expected = [

    0.0,

    momentum_step,

    -2.0 * momentum_step,

    -momentum_step,

  ]; // Represents DFT modes [0, +1, -2, -1], with index 2 containing the negative Nyquist mode.

  for momentum_index in 0..point_count {

    assert_f32_approximately_equal(

      momentum_grid[momentum_index],

      expected[momentum_index],

    );

  }

}