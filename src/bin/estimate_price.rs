// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;
extern crate linear_regression;

const DEFAULT_SAVE: &'static str = "Save.toml";
const DEFAULT_MILEAGE: &'static str = "0";

fn main () {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  println!("{}",
    linear_regression::formula::estimate_price::estimatePrice::new (
      options.value_of("mileage").unwrap_or(DEFAULT_MILEAGE)
                                 .parse::<i64>().unwrap(),
      linear_regression::save::Store::new (
        options.value_of("save").unwrap_or(DEFAULT_SAVE),
      ).ok().unwrap().get().unwrap(),
    )
  );
}
