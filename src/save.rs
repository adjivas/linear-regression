// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/linear_regression
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;
extern crate toml;

use std::io::{
  Seek,
  Read,
  Write,
};

/// The `Store` stucture is an toml interface of save.

pub struct Store {
  writted: bool,
  source: std::fs::File,
  toml: toml::Value,
}

impl Store {

  /// The `new` constructor function returns a storage's interface.

  pub fn new (
    filename: &str,
  ) -> std::io::Result<Self> {
    let mut buff: String = String::new();
    let mut source: std::fs::File = try! (
        std::fs::OpenOptions::new().read(true)
                                   .write(true)
                                   .open(filename)
    );

    try!(source.read_to_string(&mut buff));
    Ok(Store {
      writted: false,
      source: source,
      toml: buff.parse().unwrap(),
    })
  }

  /// The `get` function returns a tuple of Theta zero and one.

  pub fn get (
    &self,
  ) -> Option<(i64, i64)> {
    match (
      self.toml.lookup("Theta.zero"),
      self.toml.lookup("Theta.one"),
    ) {
      (Some(z), Some(o)) => Some((
        z.as_integer().unwrap_or(0i64),
        o.as_integer().unwrap_or(0i64),
      )),
      _ => None,
    }
  }

  /// The `set_zero` function updates the Theta zero.

  pub fn set_zero (
    &mut self,
    zero: i64,
  ) -> Option<()> {
    self.writted = true;
    match self.toml.lookup_mut("Theta.zero") {
      Some(z) => Some(*z = toml::Value::Integer(zero)),
      None => None,
    }
  }

  /// The `set_one` function updates the Theta one.

  pub fn set_one (
    &mut self,
    one: i64,
  ) -> Option<()> {
    self.writted = true;
    match self.toml.lookup_mut("Theta.one") {
      Some(o) => Some(*o = toml::Value::Integer(one)),
      None => None,
    }
  }
}

impl Drop for Store {

  /// The `drop` destructor function saves the Theta zero
  /// and one to the file.

  fn drop (
    &mut self,
  ) {
    if self.writted {
      self.source.seek(std::io::SeekFrom::Start(0)).ok().unwrap();
      self.source.write_all (
        &format!("{}", self.toml).into_bytes()[..]
      ).ok().unwrap();
    }
  }
}

impl std::fmt::Display for Store {

  /// The `fmt` function prints the Theta zero and one.

  fn fmt (
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
    match (
      self.toml.lookup("Theta.zero"),
      self.toml.lookup("Theta.one"),
    ) {
      (Some(zero), Some(one)) => write!(f, "{}, {}", zero, one),
      _ => write!(f, "nul, nul"),
    }
  }
}
