pub fn conservative_chebyshev_tail_bound( // Computes the documented analytic upper bound on the omitted Bessel-Chebyshev tail.

  bessel_argument: f64,

  max_order: usize,

) -> f64 {

  let x =

    bessel_argument.abs()

    / 2.0;

  let bound_is_valid = // The geometric-tail derivation requires M + 2 to be strictly greater than x.

    max_order as f64

    + 2.0

    > x;

  if !bound_is_valid {

    return f64::INFINITY; // Reports that this analytic formula supplies no finite certified bound at the requested order.

  }

  if x == 0.0 {

    return 0.0; // Every omitted positive-order Bessel term vanishes exactly when the Bessel argument is zero.

  }

  let log_x =

    x.ln();

  let mut log_first_omitted_term = 0.0_f64; // Accumulates log(x^(M+1) / (M+1)!) without constructing an overflowing intermediate value.

  for order in 0..=max_order {

    let factorial_factor =

      order as f64

      + 1.0;

    log_first_omitted_term +=

      log_x

      - factorial_factor.ln();

  }

  let geometric_ratio_upper_bound = // Bounds every later factorial-series term relative to the preceding one.

    x

    / (

      max_order as f64

      + 2.0

    );

  let log_tail_bound =

    2.0_f64.ln()

    + log_first_omitted_term

    - (

      1.0

      - geometric_ratio_upper_bound

    )

    .ln();

  log_tail_bound.exp()

}

pub fn minimum_chebyshev_order_for_tolerance( // Finds the smallest retained Chebyshev order whose valid conservative tail bound meets the requested tolerance.

  bessel_argument: f64,

  requested_tolerance: f64,

) -> usize {

  assert!(

    bessel_argument.is_finite(),

    "Chebyshev Bessel argument must be finite.",

  );

  assert!(

    requested_tolerance > 0.0,

    "Chebyshev truncation tolerance must be positive.",

  );

  let x =

    bessel_argument.abs()

    / 2.0;

  let mut max_order = 0_usize;

  loop {

    let bound_is_valid = // Uses the analytic tail formula only after its required geometric-series condition is satisfied.

      max_order as f64

      + 2.0

      > x;

    if bound_is_valid {

      let tail_bound = conservative_chebyshev_tail_bound(

        bessel_argument,

        max_order,

      );

      if tail_bound <= requested_tolerance {

        return max_order;

      }

    }

    max_order += 1;

  }

}