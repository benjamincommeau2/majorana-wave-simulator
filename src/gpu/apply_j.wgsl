@group(0) @binding(0) // Declares that the Majorana state buffer will be supplied through bind group 0 at binding 0.

var<storage, read_write> state: array<vec4<f32>>; // Gives the shader read-and-write access to four-f32 Majorana states stored on the GPU.

@compute // Declares the following function as a GPU compute-shader entry point.

@workgroup_size(1) // Uses one GPU invocation for this first single-state correctness test.

fn main() { // Starts the compute operation that will apply J to one Majorana state.

  let input = state[0]; // Reads the first four-component Majorana state from GPU storage.

  state[0] = vec4<f32>( // Starts constructing the transformed four-component state for J = I tensor iY.

    input.y, // Applies iY to the adjacent pair [a,b], producing b as the first output component.

    -input.x, // Applies iY to the adjacent pair [a,b], producing -a as the second output component.

    input.w, // Applies the same iY action to the adjacent pair [c,d], producing d as the third output component.

    -input.z, // Applies the same iY action to the adjacent pair [c,d], producing -c as the fourth output component.

  ); // Finishes the transformed Majorana state and writes it back into GPU storage.

} // Closes the compute-shader entry point.