use crate::physics::dirac_generator::direct_dirac_generator_1d; // Reuses the tested complete one-dimensional real Majorana time generator K.

pub fn direct_real_chebyshev_basis_1d( // Constructs real Chebyshev basis fields Phi_0 through Phi_max_order for the trusted CPU reference path.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass: f32,

  spectral_scale: f32,

  max_order: usize,

) -> Vec<Vec<[f32; 4]>> {

  assert!( // Rejects an empty field because the underlying spectral Dirac generator requires a physical lattice.

    !field.is_empty(),

    "Chebyshev basis construction requires at least one lattice point.",

  );

  assert!( // Requires a positive scaling value because every application of the recurrence divides K by this scale.

    spectral_scale > 0.0,

    "Chebyshev spectral scale must be positive.",

  );

  let phi_zero = field.to_vec(); // Defines Phi_0 = Psi exactly as required by the real Chebyshev basis.

  let mut basis_states = Vec::with_capacity( // Allocates enough outer storage for Phi_0 through Phi_max_order.

    max_order + 1,

  );

  basis_states.push( // Stores Phi_0 as the first inspectable CPU reference basis field.

    phi_zero,

  );

  if max_order == 0 { // Stops immediately when the caller requests only the zeroth basis state.

    return basis_states;

  }

  let generator_phi_zero = direct_dirac_generator_1d( // Computes K Phi_0 using the already-tested real Majorana Dirac generator.

    &basis_states[0],

    lattice_spacing,

    mass,

  );

  let mut phi_one = Vec::with_capacity( // Allocates one scaled generated state for every spatial lattice point.

    field.len(),

  );

  for generated_state in &generator_phi_zero { // Forms Phi_1 = (K / a) Phi_0 point by point.

    phi_one.push(

      [

        generated_state[0] / spectral_scale,

        generated_state[1] / spectral_scale,

        generated_state[2] / spectral_scale,

        generated_state[3] / spectral_scale,

      ],

    );

  }

  basis_states.push( // Stores Phi_1 as the second inspectable CPU reference basis field.

    phi_one,

  );

  for order in 1..max_order { // Builds Phi_2 through Phi_max_order using the real Chebyshev recurrence.

    let generator_current = direct_dirac_generator_1d( // Computes K Phi_n for the current basis field.

      &basis_states[order],

      lattice_spacing,

      mass,

    );

    let mut next_basis_state = Vec::with_capacity( // Allocates storage for Phi_(n+1) across the complete lattice.

      field.len(),

    );

    for spatial_index in 0..field.len() { // Applies Phi_(n+1) = 2(K/a)Phi_n + Phi_(n-1) independently at every spatial point.

      let previous_state =

        basis_states[order - 1][spatial_index];

      let generated_state =

        generator_current[spatial_index];

      let next_state = [

        2.0 * generated_state[0] / spectral_scale

          + previous_state[0],

        2.0 * generated_state[1] / spectral_scale

          + previous_state[1],

        2.0 * generated_state[2] / spectral_scale

          + previous_state[2],

        2.0 * generated_state[3] / spectral_scale

          + previous_state[3],

      ];

      next_basis_state.push(

        next_state,

      );

    }

    basis_states.push( // Retains this basis field for inspection by the deliberately memory-heavy CPU oracle.

      next_basis_state,

    );

  }

  basis_states // Returns Phi_0 through Phi_max_order in increasing Chebyshev order.

}