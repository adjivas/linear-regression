// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `estimatePrice` stucture is the
/// `estimatePrice(mileage) = θ0 + (θ1 ∗ mileage)`'s equation.

pub struct estimatePrice {
  mileage: i64,
  theta: (i64, i64),
}

impl estimatePrice {

  /// The `new` constructor function returns the estimatePrice's
  /// equation.

  pub fn new (
    mileage: i64,
    theta: (i64, i64),
  ) -> Self {
    estimatePrice {
      mileage: mileage,
      theta: theta,
    }
  }
}

impl std::fmt::Display for estimatePrice {

  /// The `fmt` function prints the estimatePrice's equation.

  fn fmt (
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
    write!(f, "(estimatePrice({}) ⇒= {}+({}*{})) ⇒ {}",
      self.mileage,
      self.theta.0,
      self.theta.1,
      self.mileage,
      self.theta.0 + (self.theta.1 * self.mileage),
    )
  }
}
