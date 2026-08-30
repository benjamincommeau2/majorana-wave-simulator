use majorana_wave_simulator::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup;

#[test] // Verifies that setup automatically selects the conservative truncation order and matching coefficient count.

fn setup_selects_order_and_coefficients_from_spectral_scale_and_physics_dt() {

  let spectral_scale = 4.0_f64;

  let physics_dt = 0.5_f64;

  let setup = ChebyshevPropagationSetup::new(

    spectral_scale,

    physics_dt,

  );

  assert_eq!(

    setup.max_order(),

    10,

  );

  assert_eq!(

    setup.coefficients().len(),

    11,

  );

}

#[test] // Verifies that setup retains the spectral scale required by the Chebyshev recurrence.

fn setup_retains_spectral_scale_for_recurrence() {

  let spectral_scale = 4.0_f64;

  let physics_dt = 0.5_f64;

  let setup = ChebyshevPropagationSetup::new(

    spectral_scale,

    physics_dt,

  );

  assert_eq!(

    setup.spectral_scale(),

    spectral_scale,

  );

}