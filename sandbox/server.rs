extern crate chrono;
extern crate fern;
extern crate futures;
extern crate grpcio;
extern crate helloworld;
#[macro_use]
extern crate log;

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use helloworld::helloworld::HelloReply;
use helloworld::helloworld::HelloRequest;
use helloworld::helloworld_grpc;
use helloworld::helloworld_grpc::Greeter;
use std::{io, thread};
use std::io::Read;
use std::sync::Arc;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
  fn say_hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
    let msg = format!("Howdy {}", req.get_name());
    let mut resp = HelloReply::new();
    resp.set_message(msg);
    let f = sink
      .success(resp)
      .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
    ctx.spawn(f)
  }
}

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

  let env = Arc::new(Environment::new(1));
  let service = helloworld_grpc::create_greeter(GreeterService);
  let mut server = ServerBuilder::new(env)
    .register_service(service)
    .bind("127.0.0.1", 50051)
    .build()
    .unwrap();
  server.start();
  for &(ref host, port) in server.bind_addrs() {
    info!("listening on {}:{}", host, port);
  }
  let (tx, rx) = oneshot::channel();
  thread::spawn(move || {
    info!("Press ENTER to exit...");
    let _ = io::stdin().read(&mut [0]).unwrap();
    tx.send(())
  });
  let _ = rx.wait();
  let _ = server.shutdown().wait();
}
