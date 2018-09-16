extern crate bindgen;

use std::env;

#[derive(Debug)]
struct SqliteTypeChooser;

impl bindgen::callbacks::ParseCallbacks for SqliteTypeChooser {
  fn int_macro(&self, _name: &str, value: i64) -> Option<bindgen::callbacks::IntKind> {
    if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
      Some(bindgen::callbacks::IntKind::I32)
    } else {
      None
    }
  }
}

fn main() {
  let header_path = env::args()
    .skip(1)
    .next()
    .expect("Expected header path as arg 1");

  let bindgen_output = bindgen::builder()
    .header(header_path)
    .parse_callbacks(Box::new(SqliteTypeChooser))
    .rustfmt_bindings(true)
    .generate()
    .expect(&format!("could not run bindgen on wrapper.h"))
    .to_string();

  println!("{}", bindgen_output);
}
