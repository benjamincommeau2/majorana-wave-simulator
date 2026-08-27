use super::MouseDragRotation; // Tests the private mouse-rotation behavior from inside its own module.

fn assert_close(actual: f32, expected: f32) { // Compares floating-point rotation values with a small tolerance.

  assert!( // Verifies that ordinary floating-point rounding stays within the expected tolerance.

    (actual - expected).abs() < 0.000_001, // Accepts only a very small difference between the calculated and expected angle.

    "expected {expected}, got {actual}", // Reports both values if the rotation calculation differs unexpectedly.

  ); // Finishes the floating-point comparison.

} // Finishes the reusable angle assertion.

#[test] // Verifies the established initial development-cube orientation.

fn new_rotation_has_expected_initial_angles() {

  let state = MouseDragRotation::new(); // Creates the same rotation state used by the browser.

  let [yaw, pitch] = state.angles(); // Reads the orientation exposed to the renderer.

  assert_close(yaw, 0.65); // Confirms the established initial horizontal angle.

  assert_close(pitch, 0.45); // Confirms the established initial vertical angle.

}

#[test] // Verifies that ordinary pointer movement does nothing until a drag begins.

fn movement_without_dragging_does_not_change_angles() {

  let mut state = MouseDragRotation::new(); // Creates an inactive mouse-rotation state.

  state.drag_to(100, 100); // Simulates pointer movement without first pressing the mouse button.

  let [yaw, pitch] = state.angles(); // Reads the orientation after the ignored movement.

  assert_close(yaw, 0.65); // Confirms horizontal rotation did not change.

  assert_close(pitch, 0.45); // Confirms vertical rotation did not change.

}

#[test] // Verifies that mouse displacement changes both rotation angles using the established sensitivity.

fn active_drag_changes_yaw_and_pitch() {

  let mut state = MouseDragRotation::new(); // Creates the initial mouse-rotation state.

  state.start_drag(100, 100); // Begins the drag from a known browser-pixel position.

  state.drag_to(110, 90); // Moves ten pixels right and ten pixels upward.

  let [yaw, pitch] = state.angles(); // Reads the resulting cube orientation.

  assert_close(yaw, 0.75); // Confirms ten horizontal pixels add 0.10 radians to yaw.

  assert_close(pitch, 0.35); // Confirms ten upward pixels subtract 0.10 radians from pitch.

}

#[test] // Verifies that extreme vertical movement cannot exceed the established pitch limits.

fn pitch_is_clamped_to_allowed_range() {

  let mut state = MouseDragRotation::new(); // Creates the initial mouse-rotation state.

  state.start_drag(0, 0); // Begins a drag from the origin.

  state.drag_to(0, 1000); // Attempts to rotate far beyond the positive pitch limit.

  let [_, upper_pitch] = state.angles(); // Reads the positively clamped pitch.

  assert_close(upper_pitch, 1.4); // Confirms the upper pitch limit is enforced.

  state.drag_to(0, -1000); // Attempts to rotate far beyond the negative pitch limit.

  let [_, lower_pitch] = state.angles(); // Reads the negatively clamped pitch.

  assert_close(lower_pitch, -1.4); // Confirms the lower pitch limit is enforced.

}

#[test] // Verifies that releasing the mouse prevents later movement from rotating the cube.

fn movement_after_stop_drag_does_not_change_angles() {

  let mut state = MouseDragRotation::new(); // Creates the initial mouse-rotation state.

  state.start_drag(100, 100); // Starts an active drag.

  state.drag_to(110, 110); // Changes the orientation once while dragging is active.

  state.stop_drag(); // Simulates releasing the mouse button.

  let angles_before_move = state.angles(); // Saves the orientation at the moment dragging stops.

  state.drag_to(300, 300); // Simulates later pointer movement with no active drag.

  assert_eq!(state.angles(), angles_before_move); // Confirms the inactive movement left both angles unchanged.

}