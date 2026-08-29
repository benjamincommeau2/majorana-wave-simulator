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

  let mut basis_states = Vec::with_capacity(

    max_order + 1,

  );

  basis_states.push(

    phi_zero,

  );

  if max_order == 0 {

    return basis_states;

  }

  let generator_phi_zero = direct_dirac_generator_1d( // Computes K Phi_0 using the already-tested real Majorana Dirac generator.

    &basis_states[0],

    lattice_spacing,

    mass,

  );

  let mut phi_one = Vec::with_capacity(

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

  basis_states.push(

    phi_one,

  );

  for order in 1..max_order { // Builds Phi_2 through Phi_max_order using the real Chebyshev recurrence.

    let generator_current = direct_dirac_generator_1d(

      &basis_states[order],

      lattice_spacing,

      mass,

    );

    let mut next_basis_state = Vec::with_capacity(

      field.len(),

    );

    for spatial_index in 0..field.len() {

      let previous_state =

        basis_states[order - 1][spatial_index];

      let generated_state =

        generator_current[spatial_index];

      let next_state = [ // Applies Phi_(n+1) = 2(K/a)Phi_n + Phi_(n-1).

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

    basis_states.push(

      next_basis_state,

    );

  }

  basis_states

}

pub fn precompute_chebyshev_coefficients( // Computes the Bessel weights once for a fixed spectral scale, physics timestep, and truncation order.

  spectral_scale: f64,

  physics_dt: f64,

  max_order: usize,

) -> Vec<f64> {

  assert!(

    spectral_scale > 0.0,

    "Chebyshev spectral scale must be positive.",

  );

  assert!(

    physics_dt >= 0.0,

    "Chebyshev physics timestep must be nonnegative.",

  );

  assert!(

    max_order <= i32::MAX as usize,

    "Chebyshev order exceeds the range supported by libm::jn.",

  );

  let argument = // Computes the fixed dimensionless Bessel argument z = a times delta t.

    spectral_scale

    * physics_dt;

  let mut coefficients = Vec::with_capacity(

    max_order + 1,

  );

  for order in 0..=max_order {

    let bessel_value = libm::jn( // Delegates the integer-order Bessel J calculation to libm during setup.

      order as i32,

      argument,

    );

    let coefficient = if order == 0 {

      bessel_value

    } else {

      2.0

        * bessel_value

    };

    coefficients.push(

      coefficient,

    );

  }

  coefficients

}

pub fn direct_real_chebyshev_propagate_1d( // Applies the Bessel-weighted real Chebyshev expansion using coefficients that were already computed during simulation setup.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass: f32,

  spectral_scale: f32,

  coefficients: &[f64],

) -> Vec<[f32; 4]> {

  assert!( // Requires at least the zeroth-order coefficient so the expansion always contains Phi_0.

    !coefficients.is_empty(),

    "Chebyshev propagation requires at least the zeroth-order coefficient.",

  );

  let max_order = // Infers the requested Chebyshev truncation order directly from the reusable coefficient array.

    coefficients.len()

    - 1;

  let basis_states = direct_real_chebyshev_basis_1d( // Builds the inspectable CPU-reference basis fields required by this coefficient array.

    field,

    lattice_spacing,

    mass,

    spectral_scale,

    max_order,

  );

  let mut accumulated_field = vec![ // Uses f64 only for the small scalar weighted accumulation so the CPU oracle loses less precision.

    [0.0_f64; 4];

    field.len()

  ];

  for order in 0..=max_order { // Adds c_n Phi_n for every retained Chebyshev order.

    let coefficient =

      coefficients[order];

    for spatial_index in 0..field.len() {

      accumulated_field[spatial_index][0] +=

        coefficient

        * basis_states[order][spatial_index][0] as f64;

      accumulated_field[spatial_index][1] +=

        coefficient

        * basis_states[order][spatial_index][1] as f64;

      accumulated_field[spatial_index][2] +=

        coefficient

        * basis_states[order][spatial_index][2] as f64;

      accumulated_field[spatial_index][3] +=

        coefficient

        * basis_states[order][spatial_index][3] as f64;

    }

  }

  let mut propagated_field = Vec::with_capacity(

    field.len(),

  );

  for accumulated_state in accumulated_field { // Converts the high-precision CPU accumulation back to the simulator's four-f32 field representation.

    propagated_field.push(

      [

        accumulated_state[0] as f32,

        accumulated_state[1] as f32,

        accumulated_state[2] as f32,

        accumulated_state[3] as f32,

      ],

    );

  }

  propagated_field

}