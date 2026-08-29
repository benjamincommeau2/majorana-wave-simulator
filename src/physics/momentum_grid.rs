pub fn create_momentum_grid_1d( // Creates signed physical momentum values in the same unshifted array order produced by the J-DFT.

  point_count: usize,

  lattice_spacing: f32,

) -> Vec<f32> {

  assert!( // Rejects a lattice with no spatial samples because its physical domain length would be zero.

    point_count > 0,

    "Momentum grid requires at least one lattice point.",

  );

  assert!( // Rejects zero or negative spacing because the physical momentum scale requires a positive lattice spacing.

    lattice_spacing > 0.0,

    "Momentum grid requires a positive lattice spacing.",

  );

  let domain_length = // Computes the periodic physical domain length L = N times delta x.

    point_count as f32

    * lattice_spacing;

  let momentum_step = // Computes the fundamental spectral spacing delta k = 2 pi divided by L.

    std::f32::consts::TAU

    / domain_length;

  let highest_nonnegative_mode_index = // Finds the final array index represented as a nonnegative Fourier mode.

    (point_count - 1)

    / 2;

  let mut momentum_grid = Vec::with_capacity( // Allocates exactly one momentum value for every J-DFT coefficient.

    point_count,

  );

  for momentum_index in 0..point_count { // Visits every coefficient in ordinary unshifted DFT array order.

    let signed_mode = if momentum_index <= highest_nonnegative_mode_index { // Keeps the low-frequency beginning of the DFT array as zero and positive modes.

      momentum_index as isize

    } else { // Interprets the upper portion of the DFT array as wrapped negative-frequency modes.

      momentum_index as isize - point_count as isize

    };

    let momentum = // Converts the signed integer Fourier mode into the physical wave number k.

      signed_mode as f32

      * momentum_step;

    momentum_grid.push(momentum); // Stores k at exactly the same array index as its corresponding J-Fourier coefficient.

  }

  momentum_grid // Returns the complete momentum grid in unshifted DFT order.

}