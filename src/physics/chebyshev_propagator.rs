use crate::physics::dirac_generator::direct_dirac_generator_1d; // Reuses the tested scalar-mass one-dimensional Majorana generator.

use crate::physics::dirac_generator::direct_dirac_generator_with_mass_profile_1d; // Reuses the tested spatial-mass Majorana generator for frozen piecewise mass profiles.

pub fn direct_real_chebyshev_basis_1d( // Constructs real Chebyshev basis fields Phi_0 through Phi_max_order for the scalar-mass CPU reference path.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass: f32,

  spectral_scale: f32,

  max_order: usize,

) -> Vec<Vec<[f32; 4]>> {

  assert!(

    !field.is_empty(),

    "Chebyshev basis construction requires at least one lattice point.",

  );

  assert!(

    spectral_scale > 0.0,

    "Chebyshev spectral scale must be positive.",

  );

  let phi_zero = field.to_vec();

  let mut basis_states = Vec::with_capacity(

    max_order + 1,

  );

  basis_states.push(

    phi_zero,

  );

  if max_order == 0 {

    return basis_states;

  }

  let generator_phi_zero = direct_dirac_generator_1d(

    &basis_states[0],

    lattice_spacing,

    mass,

  );

  let mut phi_one = Vec::with_capacity(

    field.len(),

  );

  for generated_state in &generator_phi_zero {

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

  for order in 1..max_order {

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

    basis_states.push(

      next_basis_state,

    );

  }

  basis_states

}

pub fn direct_real_chebyshev_basis_with_mass_profile_1d( // Constructs the same real Chebyshev basis while using one frozen spatial mass value per lattice site.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass_profile: &[f32],

  spectral_scale: f32,

  max_order: usize,

) -> Vec<Vec<[f32; 4]>> {

  assert!(

    !field.is_empty(),

    "Chebyshev basis construction requires at least one lattice point.",

  );

  assert_eq!(

    mass_profile.len(),

    field.len(),

    "Mass profile length must match the Majorana field length.",

  );

  assert!(

    spectral_scale > 0.0,

    "Chebyshev spectral scale must be positive.",

  );

  let phi_zero = field.to_vec(); // Preserves the incoming quantum state exactly as Phi_0.

  let mut basis_states = Vec::with_capacity(

    max_order + 1,

  );

  basis_states.push(

    phi_zero,

  );

  if max_order == 0 {

    return basis_states;

  }

  let generator_phi_zero = direct_dirac_generator_with_mass_profile_1d( // Applies the current frozen mass profile to the original state.

    &basis_states[0],

    lattice_spacing,

    mass_profile,

  );

  let mut phi_one = Vec::with_capacity(

    field.len(),

  );

  for generated_state in &generator_phi_zero { // Forms Phi_1 = K[m(x)] Phi_0 / a.

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

  for order in 1..max_order { // Reuses the same frozen mass profile for every generator application during this Chebyshev expansion.

    let generator_current = direct_dirac_generator_with_mass_profile_1d(

      &basis_states[order],

      lattice_spacing,

      mass_profile,

    );

    let mut next_basis_state = Vec::with_capacity(

      field.len(),

    );

    for spatial_index in 0..field.len() {

      let previous_state =

        basis_states[order - 1][spatial_index];

      let generated_state =

        generator_current[spatial_index];

      let next_state = [ // Applies Phi_(n+1) = 2 K[m(x)] Phi_n / a + Phi_(n-1).

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

  let argument =

    spectral_scale

    * physics_dt;

  let mut coefficients = Vec::with_capacity(

    max_order + 1,

  );

  for order in 0..=max_order {

    let bessel_value = libm::jn(

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

pub fn direct_real_chebyshev_propagate_1d( // Applies the scalar-mass Bessel-weighted real Chebyshev expansion using coefficients already computed during simulation setup.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass: f32,

  spectral_scale: f32,

  coefficients: &[f64],

) -> Vec<[f32; 4]> {

  assert!(

    !coefficients.is_empty(),

    "Chebyshev propagation requires at least the zeroth-order coefficient.",

  );

  let max_order =

    coefficients.len()

    - 1;

  let basis_states = direct_real_chebyshev_basis_1d(

    field,

    lattice_spacing,

    mass,

    spectral_scale,

    max_order,

  );

  let mut accumulated_field = vec![

    [0.0_f64; 4];

    field.len()

  ];

  for order in 0..=max_order {

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

  for accumulated_state in accumulated_field {

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

pub fn direct_real_chebyshev_propagate_with_mass_profile_1d( // Applies the Bessel-weighted real Chebyshev expansion using one frozen spatial mass profile.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass_profile: &[f32],

  spectral_scale: f32,

  coefficients: &[f64],

) -> Vec<[f32; 4]> {

  assert!(

    !coefficients.is_empty(),

    "Chebyshev propagation requires at least the zeroth-order coefficient.",

  );

  let max_order =

    coefficients.len()

    - 1;

  let basis_states = direct_real_chebyshev_basis_with_mass_profile_1d( // Builds every CPU-reference basis state using the same frozen Hamiltonian for this propagation interval.

    field,

    lattice_spacing,

    mass_profile,

    spectral_scale,

    max_order,

  );

  let mut accumulated_field = vec![

    [0.0_f64; 4];

    field.len()

  ];

  for order in 0..=max_order { // Applies the already-precomputed Bessel coefficient belonging to each Chebyshev basis order.

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

  for accumulated_state in accumulated_field {

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