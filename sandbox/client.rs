extern crate chrono;
extern crate fern;
extern crate grpcio;
extern crate helloworld;
#[macro_use]
extern crate log;

use grpcio::{ChannelBuilder, EnvBuilder};
use helloworld::helloworld::HelloReply;
use helloworld::helloworld::HelloRequest;
use helloworld::helloworld_grpc::GreeterClient;
use std::sync::Arc;

fn main() {
  fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            ::chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LogLevelFilter::Debug)
    // - and per-module overrides
    .chain(std::io::stdout())
    // Apply globally
    .apply()
    .unwrap();

  let env = Arc::new(EnvBuilder::new().build());
  let ch = ChannelBuilder::new(env).connect("localhost:50051");
  let client = GreeterClient::new(ch);

  let mut req = HelloRequest::new();
  req.set_name("Pardner".to_owned());
  let reply = client.say_hello(&req).expect("rpc");
  info!("Greeter received: {}", reply.get_message());
}
