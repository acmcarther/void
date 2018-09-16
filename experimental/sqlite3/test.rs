extern crate init;
#[macro_use]
extern crate log;
extern crate rusqlite;

use rusqlite::Connection;

fn main() {
  init::init();

  let conn = Connection::open_in_memory().unwrap();
}
