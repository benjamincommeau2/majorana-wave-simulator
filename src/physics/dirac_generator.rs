use crate::physics::dirac_basis::apply_alpha_x; // Reuses the tested alpha_x component mapping from the locked Majorana Dirac basis.

use crate::physics::dirac_basis::apply_minus_i_beta; // Reuses the tested real matrix B = -i beta for the Majorana mass term.

use crate::physics::spectral_derivative::direct_spectral_derivative_1d; // Reuses the independently validated direct CPU spectral first derivative.

pub fn direct_dirac_kinetic_generator_x_1d( // Applies the one-dimensional real Majorana kinetic time generator K_x = -alpha_x partial_x.

  field: &[[f32; 4]],

  lattice_spacing: f32,

) -> Vec<[f32; 4]> {

  let spatial_derivative = direct_spectral_derivative_1d( // Computes partial_x Psi using the trusted direct J-DFT spectral derivative reference.

    field,

    lattice_spacing,

  );

  let mut kinetic_generator = Vec::with_capacity( // Allocates exactly one generated Majorana state for every spatial lattice point.

    spatial_derivative.len(),

  );

  for derivative_state in &spatial_derivative { // Applies the local alpha_x matrix independently at every spatial point.

    let alpha_x_derivative_state = apply_alpha_x( // Computes alpha_x partial_x Psi using the tested Dirac-basis operation.

      derivative_state,

    );

    let generated_state = [ // Applies the minus sign required by K_x = -alpha_x partial_x.

      -alpha_x_derivative_state[0],

      -alpha_x_derivative_state[1],

      -alpha_x_derivative_state[2],

      -alpha_x_derivative_state[3],

    ];

    kinetic_generator.push(

      generated_state,

    );

  }

  kinetic_generator

}

pub fn apply_dirac_mass_generator( // Applies the local real Majorana mass time generator K_m = m(-i beta).

  state: &[f32; 4],

  mass: f32,

) -> [f32; 4] {

  let minus_i_beta_state = apply_minus_i_beta(

    state,

  );

  [

    mass * minus_i_beta_state[0],

    mass * minus_i_beta_state[1],

    mass * minus_i_beta_state[2],

    mass * minus_i_beta_state[3],

  ]

}

pub fn direct_dirac_generator_1d( // Applies the complete one-dimensional real Majorana generator for one spatially uniform mass.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass: f32,

) -> Vec<[f32; 4]> {

  let kinetic_generator = direct_dirac_kinetic_generator_x_1d( // Computes -alpha_x partial_x Psi across the complete field.

    field,

    lattice_spacing,

  );

  let mut complete_generator = Vec::with_capacity(

    field.len(),

  );

  for spatial_index in 0..field.len() {

    let mass_generator = apply_dirac_mass_generator( // Applies the same scalar mass at every lattice point.

      &field[spatial_index],

      mass,

    );

    let generated_state = [

      kinetic_generator[spatial_index][0] + mass_generator[0],

      kinetic_generator[spatial_index][1] + mass_generator[1],

      kinetic_generator[spatial_index][2] + mass_generator[2],

      kinetic_generator[spatial_index][3] + mass_generator[3],

    ];

    complete_generator.push(

      generated_state,

    );

  }

  complete_generator

}

pub fn direct_dirac_generator_with_mass_profile_1d( // Applies the complete one-dimensional generator using an independently specified mass at every lattice site.

  field: &[[f32; 4]],

  lattice_spacing: f32,

  mass_profile: &[f32],

) -> Vec<[f32; 4]> {

  assert_eq!( // Requires every physical lattice site to have exactly one corresponding mass value.

    mass_profile.len(),

    field.len(),

    "Mass profile length must match the Majorana field length.",

  );

  let kinetic_generator = direct_dirac_kinetic_generator_x_1d( // Computes the mass-independent kinetic contribution once across the complete field.

    field,

    lattice_spacing,

  );

  let mut complete_generator = Vec::with_capacity(

    field.len(),

  );

  for spatial_index in 0..field.len() { // Combines the shared spectral kinetic term with the local mass value at each lattice site.

    let local_mass =

      mass_profile[spatial_index];

    let mass_generator = apply_dirac_mass_generator( // Computes m(x) B Psi(x) using the mass currently assigned to this particular site.

      &field[spatial_index],

      local_mass,

    );

    let generated_state = [

      kinetic_generator[spatial_index][0] + mass_generator[0],

      kinetic_generator[spatial_index][1] + mass_generator[1],

      kinetic_generator[spatial_index][2] + mass_generator[2],

      kinetic_generator[spatial_index][3] + mass_generator[3],

    ];

    complete_generator.push(

      generated_state,

    );

  }

  complete_generator

}