#![feature(used)]
#[macro_use]
extern crate lazy_static;
extern crate netcode_io_sys as nio;
#[macro_use]
extern crate zcfg;

pub mod flags {
  define_pub_cfg!(
    nio_server_addr,
    String,
    "127.0.0.1:40000".to_owned(),
    "ADDRESS:PORT for the netcode io server"
  );

  define_pub_cfg!(
    max_connection_count,
    u32,
    16u32,
    "Max number of accepted connections"
  );

  define_pub_cfg!(
    max_unfragmented_packet_size,
    u32,
    ::nio::NETCODE_MAX_PACKET_SIZE as u32,
    "Maximum size of an unfragmented packet (else fragmentation required)."
  );
}

pub const NO_CLIENT_CONNECTED: i64 = -1;
pub const NO_CLIENT_SLOT: i64 = -1;

pub type ClientId = i64;
pub type ClientSlot = usize;

pub trait NetcodeServer {
  fn get_connected_clients(&self) -> Vec<ClientId>;
  fn send_packet(&mut self, client_id: &ClientId, payload_bytes: Vec<u8>);
  fn update(&mut self);
  fn retrieve_packets(&mut self, client_id: &ClientId) -> Vec<Vec<u8>>;
}

pub trait NetcodeClient {
  fn is_connected(&self) -> bool;
  fn send_packet(&self, payload_bytes: Vec<u8>);
  fn update(&mut self);
  fn retrieve_packets(&mut self) -> Vec<Vec<u8>>;
}
