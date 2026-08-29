pub fn apply_alpha_x(state: &[f32; 4]) -> [f32; 4] { // Applies alpha-x = X tensor I to one four-component real Majorana spinor.

  [ // Starts the transformed four-component state.

    state[2], // Moves c into the first component because X swaps the two spectator-qubit blocks.

    state[3], // Moves d into the second component while preserving the adjacent internal J pair.

    state[0], // Moves a into the third component.

    state[1], // Moves b into the fourth component.

  ] // Finishes alpha-x(a,b,c,d) = (c,d,a,b).

} // Closes the alpha-x implementation.

pub fn apply_alpha_y(state: &[f32; 4]) -> [f32; 4] { // Applies alpha-y = Y tensor Y to one four-component real Majorana spinor.

  [ // Starts the transformed four-component state.

    -state[3], // Produces -d as the first output component.

    state[2], // Produces c as the second output component.

    state[1], // Produces b as the third output component.

    -state[0], // Produces -a as the fourth output component.

  ] // Finishes alpha-y(a,b,c,d) = (-d,c,b,-a).

} // Closes the alpha-y implementation.

pub fn apply_alpha_z(state: &[f32; 4]) -> [f32; 4] { // Applies alpha-z = Z tensor I to one four-component real Majorana spinor.

  [ // Starts the transformed four-component state.

    state[0], // Keeps a unchanged because Z gives +1 on the first spectator-qubit state.

    state[1], // Keeps b unchanged for the same +1 block.

    -state[2], // Negates c because Z gives -1 on the second spectator-qubit state.

    -state[3], // Negates d for the same -1 block.

  ] // Finishes alpha-z(a,b,c,d) = (a,b,-c,-d).

} // Closes the alpha-z implementation.

pub fn apply_minus_i_beta(state: &[f32; 4]) -> [f32; 4] { // Applies the real mass-generator matrix -i beta = -i times Y tensor Z to one real Majorana spinor.

  [ // Starts the transformed four-component state.

    -state[2], // Produces -c as the first output component.

    state[3], // Produces d as the second output component.

    state[0], // Produces a as the third output component.

    -state[1], // Produces -b as the fourth output component.

  ] // Finishes (-i beta)(a,b,c,d) = (-c,d,a,-b).

} // Closes the real -i beta implementation.