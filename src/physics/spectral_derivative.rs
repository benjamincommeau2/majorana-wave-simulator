use crate::physics::j::apply_j; // Reuses the tested real complex structure J when multiplying Fourier coefficients by J k.

use crate::physics::j_fourier_transform::direct_forward_j_dft; // Uses the deliberately slow trusted CPU forward J-DFT reference implementation.

use crate::physics::j_fourier_transform::direct_inverse_j_dft; // Uses the deliberately slow trusted CPU inverse J-DFT reference implementation.

use crate::physics::momentum_grid::create_momentum_grid_1d; // Uses the tested signed momentum convention matching J-DFT coefficient ordering.

pub fn direct_spectral_derivative_1d( // Computes the one-dimensional pseudospectral first derivative through the direct CPU J-DFT reference path.

  field: &[[f32; 4]],

  lattice_spacing: f32,

) -> Vec<[f32; 4]> {

  let transformed = direct_forward_j_dft( // Converts the real spatial Majorana field into J-Fourier coefficient space.

    field,

  );

  let momentum_grid = create_momentum_grid_1d( // Creates the physical k value corresponding to every J-Fourier coefficient.

    field.len(),

    lattice_spacing,

  );

  let mut derivative_coefficients = vec![ // Allocates one four-component spectral derivative coefficient for every Fourier mode.

    [0.0; 4];

    transformed.len()

  ];

  for frequency_index in 0..transformed.len() { // Visits every discrete Fourier mode independently.

    let is_even_grid = // Records whether this lattice contains a single ambiguous Nyquist coefficient.

      transformed.len() % 2 == 0;

    let is_nyquist_mode = // Identifies the halfway DFT slot that represents the even-grid Nyquist frequency.

      is_even_grid

      && frequency_index == transformed.len() / 2;

    let derivative_momentum = if is_nyquist_mode { // Applies the symmetric first-derivative convention only to the ambiguous Nyquist mode.

      0.0

    } else { // Uses the ordinary signed physical momentum for every non-Nyquist Fourier mode.

      momentum_grid[frequency_index]

    };

    let j_transformed_state = apply_j( // Applies J because differentiation of exp(J k x) multiplies the Fourier mode by J k.

      &transformed[frequency_index],

    );

    derivative_coefficients[frequency_index] = [ // Forms J k times the Fourier coefficient, using zero k only for the even-grid Nyquist first derivative.

      derivative_momentum * j_transformed_state[0],

      derivative_momentum * j_transformed_state[1],

      derivative_momentum * j_transformed_state[2],

      derivative_momentum * j_transformed_state[3],

    ];

  }

  direct_inverse_j_dft( // Converts the differentiated J-Fourier coefficients back into the real spatial derivative field.

    &derivative_coefficients,

  )

}