use majorana_wave_simulator::physics::j::apply_j; // Imports the trusted J implementation so Fourier tests can compare against the already-tested real complex structure.

use majorana_wave_simulator::physics::j_fourier_transform::apply_j_rotation; // Imports the tested J rotation exp(J theta).

use majorana_wave_simulator::physics::j_fourier_transform::direct_forward_j_dft; // Imports the intentionally slow direct forward J-DFT reference implementation.

use majorana_wave_simulator::physics::j_fourier_transform::direct_inverse_j_dft; // Imports the direct inverse J-DFT used to reconstruct the original real Majorana field.

fn assert_state_approximately_equal( // Compares two floating-point Majorana states while allowing small rounding errors from trigonometric calculations.

  actual: &[f32; 4],

  expected: &[f32; 4],

) {

  let tolerance = 1.0e-5_f32; // Uses a small tolerance appropriate for these simple f32 reference calculations.

  for component_index in 0..4 { // Checks all four real Majorana components independently.

    let difference = (actual[component_index] - expected[component_index]).abs(); // Measures the absolute component error.

    assert!( // Requires this component to remain within the allowed numerical tolerance.

      difference < tolerance,

      "component {component_index} differed: actual = {}, expected = {}",

      actual[component_index],

      expected[component_index],

    );

  }

}

fn negate_state(state: &[f32; 4]) -> [f32; 4] { // Creates minus a Majorana state for exact Fourier phase expectations.

  [

    -state[0],

    -state[1],

    -state[2],

    -state[3],

  ]

}

#[test]

fn zero_angle_j_rotation_returns_original_state() {

  let state = [1.0, 2.0, 3.0, 4.0];

  let rotated = apply_j_rotation(

    &state,

    0.0,

  );

  assert_state_approximately_equal(

    &rotated,

    &state,

  );

}

#[test]

fn quarter_turn_j_rotation_matches_apply_j() {

  let state = [1.0, 2.0, 3.0, 4.0];

  let rotated = apply_j_rotation(

    &state,

    std::f32::consts::FRAC_PI_2,

  );

  let expected = apply_j(&state);

  assert_state_approximately_equal(

    &rotated,

    &expected,

  );

}

#[test]

fn half_turn_j_rotation_negates_state() {

  let state = [1.0, 2.0, 3.0, 4.0];

  let rotated = apply_j_rotation(

    &state,

    std::f32::consts::PI,

  );

  let expected = [-1.0, -2.0, -3.0, -4.0];

  assert_state_approximately_equal(

    &rotated,

    &expected,

  );

}

#[test]

fn full_turn_j_rotation_returns_original_state() {

  let state = [1.0, 2.0, 3.0, 4.0];

  let rotated = apply_j_rotation(

    &state,

    std::f32::consts::TAU,

  );

  assert_state_approximately_equal(

    &rotated,

    &state,

  );

}

#[test]

fn constant_field_transforms_only_to_zero_frequency() {

  let constant_state = [1.0, 2.0, 3.0, 4.0];

  let field = [

    constant_state,

    constant_state,

    constant_state,

    constant_state,

  ];

  let transformed = direct_forward_j_dft(

    &field,

  );

  assert_eq!(

    transformed.len(),

    4,

  );

  assert_state_approximately_equal(

    &transformed[0],

    &[4.0, 8.0, 12.0, 16.0],

  );

  assert_state_approximately_equal(

    &transformed[1],

    &[0.0, 0.0, 0.0, 0.0],

  );

  assert_state_approximately_equal(

    &transformed[2],

    &[0.0, 0.0, 0.0, 0.0],

  );

  assert_state_approximately_equal(

    &transformed[3],

    &[0.0, 0.0, 0.0, 0.0],

  );

}

#[test]

fn impulse_at_spatial_index_one_has_expected_forward_j_phases() {

  let state = [1.0, 2.0, 3.0, 4.0];

  let zero_state = [0.0, 0.0, 0.0, 0.0];

  let field = [

    zero_state,

    state,

    zero_state,

    zero_state,

  ];

  let transformed = direct_forward_j_dft(

    &field,

  );

  let j_state = apply_j(&state);

  let negative_j_state = negate_state(&j_state);

  let negative_state = negate_state(&state);

  assert_state_approximately_equal(

    &transformed[0],

    &state,

  );

  assert_state_approximately_equal(

    &transformed[1],

    &negative_j_state,

  );

  assert_state_approximately_equal(

    &transformed[2],

    &negative_state,

  );

  assert_state_approximately_equal(

    &transformed[3],

    &j_state,

  );

}

#[test] // Verifies that the chosen forward and inverse J-DFT conventions are actual inverses of one another.

fn forward_then_inverse_j_dft_recovers_original_field() {

  let field = [ // Uses four different Majorana spinors so the round trip exercises spatial positions and all spinor components.

    [1.0, 2.0, 3.0, 4.0],

    [5.0, -2.0, 1.0, 0.5],

    [-3.0, 7.0, -1.0, 2.0],

    [0.25, -0.75, 6.0, -4.0],

  ];

  let transformed = direct_forward_j_dft( // Applies the unnormalized forward transform with the negative J phase.

    &field,

  );

  let reconstructed = direct_inverse_j_dft( // Applies the positive J phase and one-over-N normalization.

    &transformed,

  );

  assert_eq!( // Requires the inverse transform to preserve the number of spatial points.

    reconstructed.len(),

    field.len(),

  );

  for spatial_index in 0..field.len() { // Checks every reconstructed lattice point independently.

    assert_state_approximately_equal(

      &reconstructed[spatial_index],

      &field[spatial_index],

    );

  }

}