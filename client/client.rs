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
use interconnect::interconnect::ConnectToServerRequest;
use interconnect::interconnect::ConnectToServerResponse;
use interconnect::interconnect::ConnectToServerResponse_SuccessfulConnection;
use interconnect::interconnect::Connection;
use interconnect::interconnect::ConnectToServerResponse_RejectedConnection;
use interconnect::interconnect::ConnectToServerResponse_RejectedConnection_RejectionKind;
//use interconnect::interconnect::GetConnectionStatusRequest;
//use interconnect::interconnect::GetConnectionStatusResponse;
//use interconnect::interconnect::GetConnectionStatusResponse_StatusError;
//use interconnect::interconnect::ReceiveStateRequest;
//use interconnect::interconnect::ReceiveStateResponse;
//use interconnect::interconnect::RelayInputsRequest;
//use interconnect::interconnect::RelayInputsResponse;
use interconnect::interconnect_grpc;
use interconnect::interconnect_grpc::GatewayServiceClient;
use std::sync::Arc;

define_pub_cfg!(
  gateway_address,
  String,
  "localhost:8743".to_owned(),
  "Configured address for the gateway"
);

fn main() {
  init::init_void();

  let env = Arc::new(EnvBuilder::new().build());
  let gateway_addr = gateway_address::CONFIG.get_value();
  info!("Using gateway address: {}", gateway_addr);
  let ch = ChannelBuilder::new(env).connect(&gateway_addr);
  let gateway_client = GatewayServiceClient::new(ch);

  let mut req = ConnectToServerRequest::new();
  let response = gateway_client.connect_to_server(&req).unwrap();

  if response.has_success() {
    let success = response.get_success();
    let connection = success.get_connection();
    info!(
      "Successfully connected as {}, with suggested server {}",
      connection.get_client_id(),
      success.get_session_addr()
    );
  } else {
    let rejected = response.get_rejected();
    info!(
      "Failed to connect with explanation {}",
      rejected.get_explanation()
    );
  }
}
