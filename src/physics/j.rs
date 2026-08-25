pub fn apply_j(state: &[f32; 4]) -> [f32; 4] { // Defines a CPU reference function that applies our chosen real J matrix to one four-component Majorana state.
  [ // Starts the new four-component state produced by applying J.
    state[2], // Computes the first output component from the third input component according to J = iY ⊗ I.
    state[3], // Computes the second output component from the fourth input component according to J = iY ⊗ I.
    -state[0], // Computes the third output component as the negative of the first input component.
    -state[1], // Computes the fourth output component as the negative of the second input component.
  ] // Finishes the four-component output array.
} // Closes the `apply_j` function.