extern crate server;
extern crate init;

fn main() {
  init::init_void();
  server::run();
}
