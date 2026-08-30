use crate::physics::chebyshev_propagator::precompute_chebyshev_coefficients;

use crate::physics::chebyshev_truncation::minimum_chebyshev_order_for_tolerance;

pub struct ChebyshevPropagationSetup {

  spectral_scale: f64,

  coefficients: Vec<f64>,

}

impl ChebyshevPropagationSetup {

  pub fn new(

    spectral_scale: f64,

    physics_dt: f64,

  ) -> Self {

    let requested_tolerance =

      f32::EPSILON as f64;

    let bessel_argument =

      spectral_scale

      * physics_dt;

    let max_order = minimum_chebyshev_order_for_tolerance(

      bessel_argument,

      requested_tolerance,

    );

    let coefficients = precompute_chebyshev_coefficients(

      spectral_scale,

      physics_dt,

      max_order,

    );

    Self {

      spectral_scale,

      coefficients,

    }

  }

  pub fn spectral_scale(

    &self,

  ) -> f64 {

    self.spectral_scale

  }

  pub fn max_order(

    &self,

  ) -> usize {

    self.coefficients.len()

    - 1

  }

  pub fn coefficients(

    &self,

  ) -> &[f64] {

    &self.coefficients

  }

}