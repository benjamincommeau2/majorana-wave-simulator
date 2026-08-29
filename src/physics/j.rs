pub fn apply_j(state: &[f32; 4]) -> [f32; 4] { // Defines the CPU reference function for J = I tensor iY acting on one four-component real Majorana state.

  [ // Starts the transformed four-component Majorana state.

    state[1], // Applies iY to the adjacent pair [a,b], producing b as the first output component.

    -state[0], // Applies iY to the adjacent pair [a,b], producing -a as the second output component.

    state[3], // Applies the same iY action to the adjacent pair [c,d], producing d as the third output component.

    -state[2], // Applies the same iY action to the adjacent pair [c,d], producing -c as the fourth output component.

  ] // Finishes the transformed four-component state.

} // Closes the CPU J reference implementation.