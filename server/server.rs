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

use futures::Future;
use futures::sync::oneshot;
use grpcio::ClientStreamingSink;
use grpcio::Environment;
use grpcio::RequestStream;
use grpcio::RpcContext;
use grpcio::Server;
use grpcio::ServerBuilder;
use grpcio::ServerStreamingSink;
use grpcio::Service;
use grpcio::UnarySink;
use interconnect::interconnect::ConnectToServerRequest;
use interconnect::interconnect::ConnectToServerResponse;
use interconnect::interconnect::ConnectToServerResponse_SuccessfulConnection;
use interconnect::interconnect::Connection;
use interconnect::interconnect::ConnectToServerResponse_RejectedConnection;
use interconnect::interconnect::ConnectToServerResponse_RejectedConnection_RejectionKind;
use interconnect::interconnect::GetConnectionStatusRequest;
use interconnect::interconnect::GetConnectionStatusResponse;
use interconnect::interconnect::GetConnectionStatusResponse_StatusError;
use interconnect::interconnect::ReceiveStateRequest;
use interconnect::interconnect::ReceiveStateResponse;
use interconnect::interconnect::RelayInputsRequest;
use interconnect::interconnect::RelayInputsResponse;
use interconnect::interconnect_grpc;
use std::env;
use std::io;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

define_pub_cfg!(
  gateway_port,
  u32,
  8743u32,
  "Which port to run the gateway service on"
);
define_pub_cfg!(
  session_port,
  u32,
  8743u32,
  "Which port to run the session service on"
);

struct ServerSet {
  gateway_server: Server,
  session_server: Option<Server>,
}

fn start_servers(gateway_service: Service, session_service: Service) -> ServerSet {
  if gateway_port::CONFIG.get_value() == session_port::CONFIG.get_value() {
    let env = Arc::new(Environment::new(2));
    let mut server = ServerBuilder::new(env)
      .register_service(gateway_service)
      .register_service(session_service)
      .bind("127.0.0.1", gateway_port::CONFIG.get_value() as u16)
      .build()
      .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
      info!("Combined server listening on {}:{}", host, port);
    }
    ServerSet {
      gateway_server: server,
      session_server: None,
    }
  } else {
    let gateway_env = Arc::new(Environment::new(2));
    let mut gateway_server = ServerBuilder::new(gateway_env)
      .register_service(gateway_service)
      .bind("127.0.0.1", gateway_port::CONFIG.get_value() as u16)
      .build()
      .unwrap();
    gateway_server.start();
    for &(ref host, port) in gateway_server.bind_addrs() {
      info!("Gateway server listening on {}:{}", host, port);
    }

    let session_env = Arc::new(Environment::new(2));
    let mut session_server = ServerBuilder::new(session_env)
      .register_service(session_service)
      .bind("127.0.0.1", session_port::CONFIG.get_value() as u16)
      .build()
      .unwrap();
    session_server.start();
    for &(ref host, port) in session_server.bind_addrs() {
      info!("Session server listening on {}:{}", host, port);
    }

    ServerSet {
      gateway_server: gateway_server,
      session_server: Some(session_server),
    }
  }
}

struct ConnectionManager {
  next_id: u64,
  details: Vec<ConnectionDetails>,
}

impl ConnectionManager {
  pub fn new() -> ConnectionManager {
    ConnectionManager {
      next_id: 0,
      details: Vec::new(),
    }
  }

  pub fn create_connection(&mut self) -> u64 {
    let this_id = self.next_id;
    self.next_id = this_id + 1;
    self
      .details
      .insert(this_id as usize, ConnectionDetails::new());
    this_id
  }

  pub fn get_details(&self, id: u64) -> Option<ConnectionDetails> {
    self.details.get(id as usize).map(|d| d.clone())
  }
}

#[derive(Clone)]
struct ConnectionDetails {
  active: bool,
}

impl ConnectionDetails {
  pub fn new() -> ConnectionDetails {
    ConnectionDetails { active: false }
  }
}

#[derive(Clone)]
struct GatewayServiceImpl {
  // This is hardcoded for now. At some point, this would be dynamic and managed by a load balancer
  session_server_addr: String,
  connection_manager: Arc<RwLock<ConnectionManager>>,
}

impl interconnect_grpc::GatewayService for GatewayServiceImpl {
  fn connect_to_server(
    &self,
    ctx: RpcContext,
    req: ConnectToServerRequest,
    sink: UnarySink<ConnectToServerResponse>,
  ) {
    let mut response = ConnectToServerResponse::new();
    match self.connection_manager.write() {
      Ok(mut mgr) => {
        let client_id = mgr.create_connection();
        let mut success_details = ConnectToServerResponse_SuccessfulConnection::new();
        let mut connection = Connection::new();
        connection.set_client_id(client_id);
        connection.set_active(false);
        success_details.set_connection(connection);
        success_details.set_session_addr(self.session_server_addr.clone());
        response.set_success(success_details);
      },
      Err(err) => {
        error!(
          "Failed to lock connection_manager in GatewayService with {}",
          err.to_string()
        );

        let mut rejection_details = ConnectToServerResponse_RejectedConnection::new();
        rejection_details.set_kind(ConnectToServerResponse_RejectedConnection_RejectionKind::FATAL);
        rejection_details.set_explanation("Connection management trouble.".to_owned());
        response.set_rejected(rejection_details);
      },
    }

    let f = sink
      .success(response)
      .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
    ctx.spawn(f)
  }

  fn get_connection_status(
    &self,
    ctx: RpcContext,
    req: GetConnectionStatusRequest,
    sink: UnarySink<GetConnectionStatusResponse>,
  ) {
    let mut response = GetConnectionStatusResponse::new();
    match self.connection_manager.read() {
      Ok(mgr) => {
        let connection_details = mgr.get_details(req.get_client_id());
        match connection_details {
          Some(details) => {
            let mut connection = Connection::new();
            connection.set_client_id(req.get_client_id());
            connection.set_active(details.active);
            response.set_connection(connection);
          },
          None => {
            let mut error = GetConnectionStatusResponse_StatusError::new();
            // At some point, this function's visibility will be limited
            // At that point, we're not going to want to be telling people that a given connection
            // doesn't exist.
            error.set_explanation(format!("Connection details unavailable."));
            response.set_error(error);
          },
        }
      },
      Err(err) => {
        error!(
          "Failed to lock connection_manager in GatewayService with {}",
          err.to_string()
        );

        let mut error = GetConnectionStatusResponse_StatusError::new();
        error.set_explanation(format!("Connection management trouble."));
        response.set_error(error);
      },
    }
  }
}


#[derive(Clone)]
struct SessionServiceImpl {
  connection_manager: Arc<RwLock<ConnectionManager>>,
}

impl interconnect_grpc::SessionService for SessionServiceImpl {
  fn relay_inputs(
    &self,
    ctx: RpcContext,
    req_stream: RequestStream<RelayInputsRequest>,
    sink: ClientStreamingSink<RelayInputsResponse>,
  ) {
  }

  fn receive_state(
    &self,
    ctx: RpcContext,
    req: ReceiveStateRequest,
    sink: ServerStreamingSink<ReceiveStateResponse>,
  ) {
  }
}

fn main() {
  init::init_void();
  let connection_manager = Arc::new(RwLock::new(ConnectionManager::new()));
  let gateway_service = interconnect_grpc::create_gateway_service(GatewayServiceImpl {
    // This obviously won't work outside the local machine
    session_server_addr: format!("localhost:{}", session_port::CONFIG.get_value()),
    connection_manager: connection_manager.clone(),
  });
  let session_service = interconnect_grpc::create_session_service(SessionServiceImpl {
    connection_manager: connection_manager,
  });

  let mut server_set = start_servers(gateway_service, session_service);
  let (tx, rx) = oneshot::channel();
  thread::spawn(move || {
    info!("Press ENTER to exit...");
    let _ = io::stdin().read(&mut [0]).unwrap();
    tx.send(())
  });
  let _ = rx.wait();
  let _ = server_set.gateway_server.shutdown().wait();
  let _ = if let Some(mut session_server) = server_set.session_server {
    session_server.shutdown().wait()
  } else {
    Ok(())
  };
}
