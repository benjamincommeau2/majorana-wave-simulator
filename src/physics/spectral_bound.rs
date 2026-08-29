pub fn maximum_active_momentum_1d( // Returns the largest momentum magnitude that the project's first spectral derivative actually uses.

  point_count: usize,

  lattice_spacing: f32,

) -> f32 {

  assert!(

    point_count > 0,

    "Spectral momentum bound requires at least one lattice point.",

  );

  assert!(

    lattice_spacing > 0.0,

    "Spectral momentum bound requires positive lattice spacing.",

  );

  let maximum_active_mode = if point_count % 2 == 0 { // Even grids contain a Nyquist slot that this project's first derivative explicitly sets to zero.

    point_count / 2

      - 1

  } else { // Odd grids have no unique Nyquist slot, so the largest signed DFT mode remains active.

    (point_count - 1)

      / 2

  };

  let momentum_spacing =

    std::f32::consts::TAU

    / (

      point_count as f32

      * lattice_spacing

    );

  maximum_active_mode as f32

    * momentum_spacing

}

pub fn conservative_dirac_spectral_scale_1d( // Bounds every allowed frozen 1D Dirac generator using the kinetic norm plus the maximum permitted mass magnitude.

  point_count: usize,

  lattice_spacing: f32,

  maximum_mass_magnitude: f32,

) -> f32 {

  assert!(

    maximum_mass_magnitude >= 0.0,

    "Maximum mass magnitude must be nonnegative.",

  );

  let maximum_active_momentum = maximum_active_momentum_1d(

    point_count,

    lattice_spacing,

  );

  maximum_active_momentum

    + maximum_mass_magnitude

}