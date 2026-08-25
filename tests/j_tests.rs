use majorana_wave_simulator::physics::j::apply_j;

#[test]
fn applying_j_twice_negates_the_state() {
  let state = [1.0, 2.0, 3.0, 4.0];

  let once = apply_j(&state);
  let twice = apply_j(&once);

  assert_eq!(twice, [-1.0, -2.0, -3.0, -4.0]);
}

#[test] // Tells Rust that the following function is an automated test for our exact chosen J component mapping.

fn applying_j_uses_expected_component_order() { // Defines a regression test that fixes the convention J(a,b,c,d) = (c,d,-a,-b).

  let state = [1.0, 2.0, 3.0, 4.0]; // Creates a state with four distinct values so every component movement and sign change can be observed.

  let transformed = apply_j(&state); // Applies the production CPU implementation of J to the test state.

  assert_eq!(transformed, [3.0, 4.0, -1.0, -2.0]); // Requires J to use exactly the component ordering and signs defined by our chosen iY tensor I convention.

} // Closes the exact-component-order test.