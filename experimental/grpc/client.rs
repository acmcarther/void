#![feature(used)]

extern crate futures;
extern crate grpcio;
extern crate init;
extern crate interconnect;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate zcfg;

use grpcio::ChannelBuilder;
use grpcio::EnvBuilder;
use grpcio::Environment;
use grpcio::ClientCStreamReceiver;
use grpcio::ClientSStreamReceiver;
use grpcio::WriteFlags;
use grpcio::ClientCStreamSender;
use futures::Sink;
use interconnect::interconnect::FindServerRequest;
use interconnect::interconnect::FindServerResponse_RejectedConnection;
use interconnect::interconnect_grpc::GatewayServiceClient;
use std::sync::Arc;

define_pub_cfg!(
  gateway_address,
  String,
  "localhost:8743".to_owned(),
  "Configured address for the gateway"
);

struct SessionServerDetails {
  own_client_id: u64,
  suggested_server: String,
}

fn find_server_details(
  env: Arc<Environment>,
  gateway_addr: &str,
) -> Result<SessionServerDetails, FindServerResponse_RejectedConnection> {
  let ch = ChannelBuilder::new(env).connect(gateway_addr);
  let gateway_client = GatewayServiceClient::new(ch);

  let req = FindServerRequest::new();
  let mut response = gateway_client.find_server(&req).unwrap();

  if response.has_rejected() {
    let rejected = response.take_rejected();
    error!(
      "Failed to find server with explanation {}",
      rejected.get_explanation()
    );
    return Err(rejected);
  }

  let success = response.get_success();
  let connection = success.get_connection();
  info!(
    "Successfully found server as client Id {}, with suggested server {}",
    connection.get_client_id(),
    success.get_session_addr()
  );

  Ok(SessionServerDetails {
    own_client_id: connection.get_client_id(),
    suggested_server: success.get_session_addr().to_owned(),
  })
}

fn main() {
  init::init();

  let env = Arc::new(EnvBuilder::new().cq_count(4).build());
  let gateway_addr = gateway_address::CONFIG.get_value();
  info!("Using gateway address: {}", gateway_addr);
  let server_details =
    find_server_details(env.clone(), &gateway_addr).expect("to have been given a suggested server");
}
