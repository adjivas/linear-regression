// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::estimate_price::estimatePrice;

pub struct linearRegression {
  mileage: i64,
  len: i64,
  data: Vec<(i64, i64)>,
}

impl linearRegression {
  pub fn new (
    mileage: i64,
    data: Vec<(i64, i64)>,
  ) -> Self {
    linearRegression {
      mileage: mileage,
      len: data.len() as i64,
      data: data,
    }
  }

 /* pub fn theta_one (
    &self,
  ) -> i64 {
    self.data.iter().fold(((0.0, 0,0), (0.0, 0.0)), |((theta0, theta_1), (x, y)), &(km, price)|
      let theta = ((theta_0 + theta_1 * km) - price) / self.len;
      ((), (x + theta, y + theta * km))
      (acc + mileage - price / self.len),
    )
  }*/
}

impl std::fmt::Display for linearRegression {

  fn fmt (
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
      Ok(())
//    write!(f, "{}", self.theta_one())
  }
}
