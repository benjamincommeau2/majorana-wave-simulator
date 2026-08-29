use majorana_wave_simulator::physics::dirac_basis::apply_alpha_x; // Imports the production alpha-x operation for testing the chosen X tensor I convention.

use majorana_wave_simulator::physics::dirac_basis::apply_alpha_y; // Imports the production alpha-y operation for testing the chosen Y tensor Y convention.

use majorana_wave_simulator::physics::dirac_basis::apply_alpha_z; // Imports the production alpha-z operation for testing the chosen Z tensor I convention.

use majorana_wave_simulator::physics::dirac_basis::apply_minus_i_beta; // Imports the real mass-generator matrix -i beta used on the real Majorana state.

use majorana_wave_simulator::physics::j::apply_j; // Imports the chosen real complex structure J = I tensor iY so its algebra can be checked against the Dirac basis.

fn negate_state(state: &[f32; 4]) -> [f32; 4] { // Creates the additive inverse needed to test anticommutation and operators that square to minus the identity.

  [ // Starts the negated four-component state.

    -state[0],

    -state[1],

    -state[2],

    -state[3],

  ] // Finishes the negated state.

}

#[test] // Verifies the exact alpha-x component mapping.

fn alpha_x_uses_expected_component_order() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses distinct values so every component movement is visible.

  let transformed = apply_alpha_x(&state); // Applies alpha-x = X tensor I.

  assert_eq!(

    transformed,

    [3.0, 4.0, 1.0, 2.0],

  ); // Checks alpha-x(a,b,c,d) = (c,d,a,b).

}

#[test] // Verifies the exact alpha-y component mapping.

fn alpha_y_uses_expected_component_order() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses distinct values so every component movement and sign change is visible.

  let transformed = apply_alpha_y(&state); // Applies alpha-y = Y tensor Y.

  assert_eq!(

    transformed,

    [-4.0, 3.0, 2.0, -1.0],

  ); // Checks alpha-y(a,b,c,d) = (-d,c,b,-a).

}

#[test] // Verifies the exact alpha-z component mapping.

fn alpha_z_uses_expected_component_order() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses distinct values so the unchanged and sign-flipped components are visible.

  let transformed = apply_alpha_z(&state); // Applies alpha-z = Z tensor I.

  assert_eq!(

    transformed,

    [1.0, 2.0, -3.0, -4.0],

  ); // Checks alpha-z(a,b,c,d) = (a,b,-c,-d).

}

#[test] // Verifies the exact real -i beta component mapping.

fn minus_i_beta_uses_expected_component_order() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses distinct values so every component movement and sign change is visible.

  let transformed = apply_minus_i_beta(&state); // Applies the real matrix -i beta that appears in the Majorana time generator.

  assert_eq!(

    transformed,

    [-3.0, 4.0, 1.0, -2.0],

  ); // Checks (-i beta)(a,b,c,d) = (-c,d,a,-b).

}

#[test] // Verifies that all three kinetic alpha matrices square to the identity.

fn each_alpha_squared_returns_the_original_state() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so incorrect signs or permutations remain observable.

  let alpha_x_once = apply_alpha_x(&state); // Applies alpha-x once.

  let alpha_x_twice = apply_alpha_x(&alpha_x_once); // Applies alpha-x again to evaluate alpha-x squared.

  let alpha_y_once = apply_alpha_y(&state); // Applies alpha-y once.

  let alpha_y_twice = apply_alpha_y(&alpha_y_once); // Applies alpha-y again to evaluate alpha-y squared.

  let alpha_z_once = apply_alpha_z(&state); // Applies alpha-z once.

  let alpha_z_twice = apply_alpha_z(&alpha_z_once); // Applies alpha-z again to evaluate alpha-z squared.

  assert_eq!(

    alpha_x_twice,

    state,

  ); // Checks alpha-x squared = I.

  assert_eq!(

    alpha_y_twice,

    state,

  ); // Checks alpha-y squared = I.

  assert_eq!(

    alpha_z_twice,

    state,

  ); // Checks alpha-z squared = I.

}

#[test] // Verifies that every distinct pair of kinetic alpha matrices anticommutes.

fn distinct_alpha_matrices_anticommute() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so the operator-order comparison exercises all four components.

  let alpha_x_then_y = apply_alpha_y(

    &apply_alpha_x(&state),

  );

  let alpha_y_then_x = apply_alpha_x(

    &apply_alpha_y(&state),

  );

  assert_eq!(

    alpha_x_then_y,

    negate_state(&alpha_y_then_x),

  ); // Checks alpha-y alpha-x = -alpha-x alpha-y.

  let alpha_x_then_z = apply_alpha_z(

    &apply_alpha_x(&state),

  );

  let alpha_z_then_x = apply_alpha_x(

    &apply_alpha_z(&state),

  );

  assert_eq!(

    alpha_x_then_z,

    negate_state(&alpha_z_then_x),

  ); // Checks alpha-z alpha-x = -alpha-x alpha-z.

  let alpha_y_then_z = apply_alpha_z(

    &apply_alpha_y(&state),

  );

  let alpha_z_then_y = apply_alpha_y(

    &apply_alpha_z(&state),

  );

  assert_eq!(

    alpha_y_then_z,

    negate_state(&alpha_z_then_y),

  ); // Checks alpha-z alpha-y = -alpha-y alpha-z.

}

#[test] // Verifies that the real mass operator -i beta squares to minus the identity.

fn minus_i_beta_squared_negates_the_state() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so all four components participate.

  let once = apply_minus_i_beta(&state); // Applies -i beta once.

  let twice = apply_minus_i_beta(&once); // Applies -i beta a second time.

  assert_eq!(

    twice,

    negate_state(&state),

  ); // Checks (-i beta)^2 = -I.

}

#[test] // Verifies that the real mass operator anticommutes with every kinetic alpha matrix.

fn minus_i_beta_anticommutes_with_each_alpha() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so each operator-order comparison exercises all four components.

  let alpha_x_then_mass = apply_minus_i_beta(

    &apply_alpha_x(&state),

  );

  let mass_then_alpha_x = apply_alpha_x(

    &apply_minus_i_beta(&state),

  );

  assert_eq!(

    alpha_x_then_mass,

    negate_state(&mass_then_alpha_x),

  ); // Checks {-i beta, alpha-x} = 0.

  let alpha_y_then_mass = apply_minus_i_beta(

    &apply_alpha_y(&state),

  );

  let mass_then_alpha_y = apply_alpha_y(

    &apply_minus_i_beta(&state),

  );

  assert_eq!(

    alpha_y_then_mass,

    negate_state(&mass_then_alpha_y),

  ); // Checks {-i beta, alpha-y} = 0.

  let alpha_z_then_mass = apply_minus_i_beta(

    &apply_alpha_z(&state),

  );

  let mass_then_alpha_z = apply_alpha_z(

    &apply_minus_i_beta(&state),

  );

  assert_eq!(

    alpha_z_then_mass,

    negate_state(&mass_then_alpha_z),

  ); // Checks {-i beta, alpha-z} = 0.

}

#[test] // Verifies that J commutes with every kinetic alpha matrix.

fn j_commutes_with_each_alpha() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so each commutator comparison exercises all four components.

  let alpha_x_then_j = apply_j(

    &apply_alpha_x(&state),

  );

  let j_then_alpha_x = apply_alpha_x(

    &apply_j(&state),

  );

  assert_eq!(

    alpha_x_then_j,

    j_then_alpha_x,

  ); // Checks [J, alpha-x] = 0.

  let alpha_y_then_j = apply_j(

    &apply_alpha_y(&state),

  );

  let j_then_alpha_y = apply_alpha_y(

    &apply_j(&state),

  );

  assert_eq!(

    alpha_y_then_j,

    j_then_alpha_y,

  ); // Checks [J, alpha-y] = 0.

  let alpha_z_then_j = apply_j(

    &apply_alpha_z(&state),

  );

  let j_then_alpha_z = apply_alpha_z(

    &apply_j(&state),

  );

  assert_eq!(

    alpha_z_then_j,

    j_then_alpha_z,

  ); // Checks [J, alpha-z] = 0.

}

#[test] // Verifies that J anticommutes with the real mass operator -i beta.

fn j_anticommutes_with_minus_i_beta() {

  let state = [1.0, 2.0, 3.0, 4.0]; // Uses a general state so the operator-order comparison exercises all four components.

  let mass_then_j = apply_j(

    &apply_minus_i_beta(&state),

  );

  let j_then_mass = apply_minus_i_beta(

    &apply_j(&state),

  );

  assert_eq!(

    mass_then_j,

    negate_state(&j_then_mass),

  ); // Checks {J, -i beta} = 0.

}