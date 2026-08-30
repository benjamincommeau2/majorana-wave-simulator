use majorana_wave_simulator::physics::chebyshev_truncation::conservative_chebyshev_tail_bound;

use majorana_wave_simulator::physics::chebyshev_truncation::minimum_chebyshev_order_for_tolerance;

fn assert_f64_approximately_equal(

  actual: f64,

  expected: f64,

) {

  let tolerance = 1.0e-15_f64;

  let difference =

    (actual - expected).abs();

  assert!(

    difference < tolerance,

    "actual = {actual}, expected = {expected}, difference = {difference}",

  );

}

#[test] // Verifies the documented conservative Bessel-tail bound at an independently calculable argument and order.

fn conservative_tail_bound_matches_known_z_two_order_five_value() {

  let bessel_argument = 2.0_f64;

  let max_order = 5;

  let actual = conservative_chebyshev_tail_bound(

    bessel_argument,

    max_order,

  );

  let expected =

    7.0_f64

    / 2160.0_f64;

  assert_f64_approximately_equal(

    actual,

    expected,

  );

}

#[test] // Expected GREEN: zero Bessel argument has no omitted higher-order tail.

fn zero_bessel_argument_has_zero_conservative_tail_bound() {

  let bessel_argument = 0.0_f64;

  let max_order = 0;

  let actual = conservative_chebyshev_tail_bound(

    bessel_argument,

    max_order,

  );

  assert_eq!(

    actual,

    0.0,

  );

}

#[test] // Verifies that orders outside the geometric-tail validity condition do not report a misleading finite error bound.

fn invalid_geometric_tail_regime_returns_infinite_bound() {

  let bessel_argument = 6.0_f64; // Gives x = |z| / 2 = 3.

  let max_order = 0; // Gives M + 2 = 2, which does not satisfy M + 2 > x.

  let actual = conservative_chebyshev_tail_bound(

    bessel_argument,

    max_order,

  );

  assert_eq!(

    actual,

    f64::INFINITY,

  );

}

#[test] // Expected GREEN: directly verifies the conservative threshold on both sides of the selected order.

fn z_two_crosses_f32_precision_threshold_between_orders_nine_and_ten() {

  let bessel_argument = 2.0_f64;

  let requested_tolerance =

    f32::EPSILON as f64;

  let order_nine_bound = conservative_chebyshev_tail_bound(

    bessel_argument,

    9,

  );

  let order_ten_bound = conservative_chebyshev_tail_bound(

    bessel_argument,

    10,

  );

  assert!(

    order_nine_bound > requested_tolerance,

    "order 9 should remain above f32 precision: bound = {order_nine_bound}, tolerance = {requested_tolerance}",

  );

  assert!(

    order_ten_bound <= requested_tolerance,

    "order 10 should meet f32 precision: bound = {order_ten_bound}, tolerance = {requested_tolerance}",

  );

}

#[test] // Verifies that automatic truncation uses the smallest conservative order below the f32 precision scale.

fn z_two_requires_order_ten_for_f32_precision_tolerance() {

  let bessel_argument = 2.0_f64;

  let requested_tolerance =

    f32::EPSILON as f64;

  let minimum_order = minimum_chebyshev_order_for_tolerance(

    bessel_argument,

    requested_tolerance,

  );

  assert_eq!(

    minimum_order,

    10,

  );

}

#[test]

#[should_panic(expected = "Chebyshev truncation tolerance must be positive.")]

fn zero_truncation_tolerance_is_rejected() {

  minimum_chebyshev_order_for_tolerance(

    2.0,

    0.0,

  );

}

#[test]

#[should_panic(expected = "Chebyshev Bessel argument must be finite.")]

fn infinite_bessel_argument_is_rejected_for_order_selection() {

  minimum_chebyshev_order_for_tolerance(

    f64::INFINITY,

    f32::EPSILON as f64,

  );

}

#[test] // Verifies that a large finite argument can recover to a small tail after intermediate factorial-series growth.

fn large_finite_argument_tail_bound_does_not_overflow_permanently() {

  let bessel_argument = 1600.0_f64; // Gives x = 800.

  let max_order = 2200; // Gives a mathematically finite bound of approximately 8.0e-14.

  let actual = conservative_chebyshev_tail_bound(

    bessel_argument,

    max_order,

  );

  assert!(

    actual.is_finite(),

    "large finite Bessel arguments should not produce a permanently infinite bound when the final analytic bound is finite",

  );

  assert!(

    actual < f32::EPSILON as f64,

    "expected the order-2200 conservative bound to be below f32 precision, but got {actual}",

  );

}