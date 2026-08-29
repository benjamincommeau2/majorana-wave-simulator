use majorana_wave_simulator::physics::j::apply_j; // Imports the trusted real complex structure J for constructing analytic derivatives.

use majorana_wave_simulator::physics::j_fourier_transform::apply_j_rotation; // Imports the tested exp(J theta) rotation used to construct exact Fourier modes.

use majorana_wave_simulator::physics::spectral_derivative::direct_spectral_derivative_1d; // Imports the direct CPU spectral derivative built from the J-DFT reference path.

fn assert_state_approximately_equal( // Compares two floating-point Majorana states while allowing small numerical error from trigonometric and Fourier calculations.

  actual: &[f32; 4],

  expected: &[f32; 4],

) {

  let tolerance = 1.0e-4_f32; // Allows the small f32 error accumulated through the direct forward and inverse transforms.

  for component_index in 0..4 { // Checks every real Majorana component independently.

    let difference = (actual[component_index] - expected[component_index]).abs(); // Measures this component's absolute numerical error.

    assert!( // Requires the numerical derivative to agree with the independently calculated expected derivative.

      difference < tolerance,

      "component {component_index} differed: actual = {}, expected = {}",

      actual[component_index],

      expected[component_index],

    );

  }

}

#[test] // Verifies the complete J-DFT spectral derivative against an analytic positive-frequency Fourier mode.

fn spectral_derivative_matches_analytic_positive_frequency_mode() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let domain_length = point_count as f32 * lattice_spacing;

  let wave_number = std::f32::consts::TAU / domain_length;

  let base_state = [1.0, 2.0, 3.0, 4.0];

  let mut field = Vec::with_capacity(

    point_count,

  );

  let mut expected_derivative = Vec::with_capacity(

    point_count,

  );

  for spatial_index in 0..point_count {

    let position = spatial_index as f32 * lattice_spacing;

    let angle = wave_number * position;

    let state_at_position = apply_j_rotation(

      &base_state,

      angle,

    );

    let j_state_at_position = apply_j(

      &state_at_position,

    );

    let derivative_at_position = [

      wave_number * j_state_at_position[0],

      wave_number * j_state_at_position[1],

      wave_number * j_state_at_position[2],

      wave_number * j_state_at_position[3],

    ];

    field.push(state_at_position);

    expected_derivative.push(derivative_at_position);

  }

  let numerical_derivative = direct_spectral_derivative_1d(

    &field,

    lattice_spacing,

  );

  assert_eq!(

    numerical_derivative.len(),

    point_count,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &numerical_derivative[spatial_index],

      &expected_derivative[spatial_index],

    );

  }

}

#[test] // Verifies that wrapped negative DFT frequencies receive the correct negative physical momentum.

fn spectral_derivative_matches_analytic_negative_frequency_mode() {

  let point_count = 5;

  let lattice_spacing = 1.0_f32;

  let domain_length = point_count as f32 * lattice_spacing;

  let wave_number = std::f32::consts::TAU / domain_length;

  let base_state = [1.0, 2.0, 3.0, 4.0];

  let mut field = Vec::with_capacity(

    point_count,

  );

  let mut expected_derivative = Vec::with_capacity(

    point_count,

  );

  for spatial_index in 0..point_count {

    let position = spatial_index as f32 * lattice_spacing;

    let angle = -wave_number * position;

    let state_at_position = apply_j_rotation(

      &base_state,

      angle,

    );

    let j_state_at_position = apply_j(

      &state_at_position,

    );

    let derivative_at_position = [

      -wave_number * j_state_at_position[0],

      -wave_number * j_state_at_position[1],

      -wave_number * j_state_at_position[2],

      -wave_number * j_state_at_position[3],

    ];

    field.push(state_at_position);

    expected_derivative.push(derivative_at_position);

  }

  let numerical_derivative = direct_spectral_derivative_1d(

    &field,

    lattice_spacing,

  );

  assert_eq!(

    numerical_derivative.len(),

    point_count,

  );

  for spatial_index in 0..point_count {

    assert_state_approximately_equal(

      &numerical_derivative[spatial_index],

      &expected_derivative[spatial_index],

    );

  }

}

#[test] // Locks down the symmetric first-derivative convention for the ambiguous even-grid Nyquist mode.

fn even_grid_nyquist_mode_has_zero_first_spectral_derivative() {

  let lattice_spacing = 1.0_f32; // Uses unit spacing so the four-point Nyquist samples simply alternate signs.

  let base_state = [1.0, 2.0, 3.0, 4.0]; // Defines a general Majorana amplitude for the Nyquist mode.

  let field = [ // Represents the sampled Nyquist mode exp(J pi n) Psi = (-1)^n Psi.

    base_state,

    [-1.0, -2.0, -3.0, -4.0],

    base_state,

    [-1.0, -2.0, -3.0, -4.0],

  ];

  let numerical_derivative = direct_spectral_derivative_1d( // Applies the current first spectral derivative to the ambiguous Nyquist mode.

    &field,

    lattice_spacing,

  );

  let zero_state = [0.0, 0.0, 0.0, 0.0]; // Represents our chosen symmetric Nyquist first-derivative result.

  for spatial_index in 0..field.len() { // Requires every sampled derivative value to vanish.

    assert_state_approximately_equal(

      &numerical_derivative[spatial_index],

      &zero_state,

    );

  }

}