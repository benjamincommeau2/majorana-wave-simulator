pub mod j; // Makes the tested real complex structure J available through the physics module.

pub mod dirac_basis; // Makes the tested Majorana Dirac-basis operations available through the physics module.

pub mod j_fourier_transform; // Makes the direct CPU J-DFT reference implementation available through the physics module.

pub mod momentum_grid; // Makes the tested discrete spectral momentum-grid construction available through the physics module.

pub mod spectral_derivative; // Makes the tested CPU reference spectral derivative available through the physics module.

pub mod dirac_generator; // Makes the tested real Majorana Dirac time generator available through the physics module.

pub mod chebyshev_propagator; // Makes the tested real Bessel-Chebyshev propagation reference available through the physics module.

pub mod chebyshev_truncation; // Makes conservative Bessel-tail truncation calculations available during Chebyshev simulation setup.

pub mod mass_profile; // Makes spatial mass-profile construction available through the physics module.

pub mod spectral_bound; // Makes conservative spectral-scale bounds available for Chebyshev setup.