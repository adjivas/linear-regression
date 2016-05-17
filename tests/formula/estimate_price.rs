// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate linear_regression;

#[test]
fn test_estimate_price_zero () {
  assert_eq! (
    format!("{}",
      linear_regression::formula::estimate_price::estimatePrice::new (
        42,
        (0, 0),
      )
    ),
    "(estimatePrice(42) ⇒= 0+(0*42)) ⇒ 0"
  );
}

