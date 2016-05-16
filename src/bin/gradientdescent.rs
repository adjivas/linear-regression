// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate linear_regression;

fn main () {
  let mut store = linear_regression::save::Store::new("Save.toml").unwrap();

  println!("{:?}", store.set_one(55));
  println!("{:?}", store.get());
  println!("{}", store);
}
