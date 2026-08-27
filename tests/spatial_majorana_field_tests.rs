// tests/spatial_majorana_field_tests.rs

use majorana_wave_simulator::spatial_majorana_field::SpatialMajoranaField; // Tests the production spatial Majorana field through the crate's public API.

#[test] // Verifies that the first simulator field contains the requested number of three-dimensional grid points.
fn spatial_field_has_expected_number_of_grid_points() {

  let field = SpatialMajoranaField::new_centered_gaussian( // Creates the first real spatial Majorana field used by the simulator.

    16, // Uses sixteen grid points along each spatial axis.

    2.0, // Uses a modest Gaussian width so the packet is localized near the center.

  ); // Finishes creating the test field.

  assert_eq!( // Requires a 16 × 16 × 16 field to contain exactly 4,096 spatial points.

    field.len(), // Reads the number of four-component states stored by the field.

    16 * 16 * 16, // Calculates the expected number of points in the cubic grid.

  ); // Finishes checking the field size.

}

#[test] // Verifies that every spatial point still represents exactly four real Majorana components.
fn spatial_field_grid_point_has_four_components() {

  let field = SpatialMajoranaField::new_centered_gaussian(16, 2.0); // Creates the same centered Gaussian field used by the simulator.

  assert_eq!( // Requires one spatial point to expose exactly four real components.

    field.components_at(7, 7, 7).len(), // Reads the number of components at a point near the center of the even-sized grid.

    4, // Preserves the four-real-component Majorana representation.

  ); // Finishes checking the component count.

}

#[test] // Verifies that the initial packet is localized near the center instead of being spatially uniform.
fn centered_gaussian_is_larger_near_center_than_corner() {

  let field = SpatialMajoranaField::new_centered_gaussian(16, 2.0); // Creates a localized Gaussian Majorana field.

  let center_amplitude = field.components_at(7, 7, 7)[0]; // Reads the populated first Majorana component near the grid center.

  let corner_amplitude = field.components_at(0, 0, 0)[0]; // Reads the same component far from the packet center.

  assert!( // Requires localization to make the center amplitude larger than the distant corner amplitude.

    center_amplitude > corner_amplitude, // Checks the defining spatial falloff of the Gaussian envelope.

  ); // Finishes checking Gaussian localization.

}

#[test] // Verifies that the initial Gaussian envelope multiplies only the chosen first Majorana basis component.
fn gaussian_initial_state_uses_first_majorana_component() {

  let field = SpatialMajoranaField::new_centered_gaussian(16, 2.0); // Creates the initial spatial Majorana field.

  let components = field.components_at(7, 7, 7); // Reads one state near the center where the Gaussian amplitude is clearly nonzero.

  assert!(components[0] > 0.0); // Confirms the first Majorana component carries the Gaussian envelope.

  assert_eq!(components[1], 0.0); // Confirms the second Majorana component begins empty.

  assert_eq!(components[2], 0.0); // Confirms the third Majorana component begins empty.

  assert_eq!(components[3], 0.0); // Confirms the fourth Majorana component begins empty.

}