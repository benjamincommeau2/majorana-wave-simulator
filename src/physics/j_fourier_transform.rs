use crate::physics::j::apply_j; // Reuses the already-tested real complex structure J instead of duplicating its component mapping.

pub fn apply_j_rotation( // Applies the real rotation exp(J theta) to one four-component Majorana spinor.

  state: &[f32; 4],

  angle: f32,

) -> [f32; 4] {

  let cosine = angle.cos(); // Computes the scalar coefficient multiplying the original state.

  let sine = angle.sin(); // Computes the scalar coefficient multiplying J applied to the state.

  let j_state = apply_j(state); // Applies the trusted J = I tensor iY mapping once.

  [ // Combines cos(theta) Psi + sin(theta) J Psi component by component.

    cosine * state[0] + sine * j_state[0],

    cosine * state[1] + sine * j_state[1],

    cosine * state[2] + sine * j_state[2],

    cosine * state[3] + sine * j_state[3],

  ] // Finishes exp(J theta) Psi.

}

pub fn direct_forward_j_dft( // Computes the intentionally slow O(N squared) forward J-DFT used as the trusted CPU reference implementation.

  field: &[[f32; 4]],

) -> Vec<[f32; 4]> {

  let point_count = field.len(); // Records the number of one-dimensional spatial samples in the input field.

  let mut transformed = vec![ // Allocates one four-component output coefficient for every spatial input point.

    [0.0; 4];

    point_count

  ];

  for frequency_index in 0..point_count { // Computes one J-Fourier coefficient for each discrete frequency.

    let mut accumulated_state = [0.0; 4]; // Starts this frequency coefficient at zero before summing all spatial contributions.

    for spatial_index in 0..point_count { // Visits every spatial sample because this reference DFT deliberately uses the direct O(N squared) definition.

      let angle = // Computes the forward-transform phase angle -2 pi m n divided by N.

        -std::f32::consts::TAU

        * frequency_index as f32

        * spatial_index as f32

        / point_count as f32;

      let rotated_state = apply_j_rotation( // Applies exp(-J 2 pi m n / N) to this spatial Majorana state.

        &field[spatial_index],

        angle,

      );

      accumulated_state[0] += rotated_state[0]; // Adds this sample's first transformed component to the current frequency coefficient.

      accumulated_state[1] += rotated_state[1]; // Adds this sample's second transformed component to the current frequency coefficient.

      accumulated_state[2] += rotated_state[2]; // Adds this sample's third transformed component to the current frequency coefficient.

      accumulated_state[3] += rotated_state[3]; // Adds this sample's fourth transformed component to the current frequency coefficient.

    }

    transformed[frequency_index] = accumulated_state; // Stores the complete unnormalized J-Fourier coefficient for this frequency.

  }

  transformed // Returns every direct forward J-DFT coefficient.

}

pub fn direct_inverse_j_dft( // Computes the intentionally slow O(N squared) inverse J-DFT used to reconstruct the real spatial Majorana field.

  transformed: &[[f32; 4]],

) -> Vec<[f32; 4]> {

  let point_count = transformed.len(); // Records the number of discrete J-Fourier coefficients being inverted.

  let mut reconstructed = vec![ // Allocates one four-component reconstructed state for every spatial point.

    [0.0; 4];

    point_count

  ];

  for spatial_index in 0..point_count { // Reconstructs one spatial Majorana state at each lattice position.

    let mut accumulated_state = [0.0; 4]; // Starts this reconstructed spatial state at zero before summing all frequency contributions.

    for frequency_index in 0..point_count { // Visits every frequency coefficient because this reference inverse uses the direct O(N squared) definition.

      let angle = // Computes the inverse-transform phase angle +2 pi m n divided by N.

        std::f32::consts::TAU

        * frequency_index as f32

        * spatial_index as f32

        / point_count as f32;

      let rotated_state = apply_j_rotation( // Applies exp(+J 2 pi m n / N) to this frequency-space Majorana coefficient.

        &transformed[frequency_index],

        angle,

      );

      accumulated_state[0] += rotated_state[0]; // Adds this frequency contribution to the first reconstructed component.

      accumulated_state[1] += rotated_state[1]; // Adds this frequency contribution to the second reconstructed component.

      accumulated_state[2] += rotated_state[2]; // Adds this frequency contribution to the third reconstructed component.

      accumulated_state[3] += rotated_state[3]; // Adds this frequency contribution to the fourth reconstructed component.

    }

    let normalization = point_count as f32; // Converts N to f32 so the inverse transform can apply the chosen one-over-N normalization.

    reconstructed[spatial_index] = [ // Stores the normalized reconstructed spatial Majorana state.

      accumulated_state[0] / normalization,

      accumulated_state[1] / normalization,

      accumulated_state[2] / normalization,

      accumulated_state[3] / normalization,

    ];

  }

  reconstructed // Returns the reconstructed real spatial field.

}