#![feature(used)]
extern crate libc;
#[macro_use]
extern crate log;
extern crate netcode_io_sys as nio;
extern crate reliable_io_sys as rio;
extern crate common;

use common::global;
use common::consts;
use common::flags;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;
use std::time::Instant;

pub const NO_CLIENT_CONNECTED: i64 = -1;
pub const NO_CLIENT_SLOT: i64 = -1;

pub trait NetcodeClient {
  fn is_connected(&self) -> bool;
  fn send_packet(&self, payload_bytes: Vec<u8>);
  fn update(&mut self);
  fn retrieve_packets(&mut self) -> Vec<Vec<u8>>;
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
  pub private_key: [u8; 32],
  pub protocol_id: u64,
  // TODO(acmcarther): This doesn't seem to match with common::ClientId
  pub client_id: u8,
  pub server_addr: String,
  pub max_connection_count: u32,
  pub max_unfragmented_packet_size: u32,
}

pub struct Client {
  config: ClientConfig,
  connecting: bool,
  nio_client: *mut nio::netcode_client_t,
  rio_endpoint: Option<*mut rio::reliable_endpoint_t>,
  unprocessed_inbound_packets: Vec<Vec<u8>>,
  start_time: Instant,
}

impl Default for ClientConfig {
  fn default() -> ClientConfig {
    let mut client_id: u8 = 0;
    unsafe {
      nio::netcode_random_bytes(&mut client_id, 8 /* bytes */);
    }

    ClientConfig {
      private_key: consts::PRIVATE_KEY,
      protocol_id: consts::PROTOCOL_ID,
      server_addr: flags::nio_server_addr::CONFIG.get_value(),
      client_id: client_id,
      max_connection_count: flags::max_connection_count::CONFIG.get_value(),
      max_unfragmented_packet_size: flags::max_unfragmented_packet_size::CONFIG
        .get_value(),
    }
  }
}

impl Client {
  pub fn start_from_config(config: ClientConfig) -> Box<Client> {
    let mut nio_client_config: nio::netcode_client_config_t = unsafe { std::mem::uninitialized() };

    unsafe {
      nio::netcode_default_client_config(&mut nio_client_config);
    }
    let self_addr = CString::new("0.0.0.0".to_owned()).unwrap();

    let nio_client: *mut nio::netcode_client_t =
      unsafe { nio::netcode_client_create(self_addr.as_ptr(), &nio_client_config, 0.0) };

    if nio_client == std::ptr::null_mut() {
      error!("Could not create netcode_io server");
      panic!("Could not create netcode_io server");
    }

    let mut connect_token: [u8; nio::NETCODE_CONNECT_TOKEN_BYTES as usize] =
      [0; nio::NETCODE_CONNECT_TOKEN_BYTES as usize];

    let addr = CString::new(config.server_addr.clone()).unwrap();
    let mut all_addresses = [addr.as_ptr()];
    unsafe {
      let token_result = nio::netcode_generate_connect_token(
        1, /* num_server_addresses */
        all_addresses.as_mut_ptr(),
        all_addresses.as_mut_ptr(),
        common::consts::CONNECT_TOKEN_EXPIRY,
        common::consts::CONNECT_TOKEN_TIMEOUT,
        config.client_id as u64,
        config.protocol_id,
        0, /* sequence */
        config.private_key.as_ptr(),
        connect_token.as_mut_ptr(),
      );

      if token_result != (nio::NETCODE_OK as i32) {
        error!("Could not create connect token");
        panic!("Could not create connect token");
      }
    }

    unsafe {
      nio::netcode_client_connect(nio_client, connect_token.as_mut_ptr());
    }

    // Create a client without a rio endpoint (temporarily)
    // This is necesary so that we can send the client as the context argument to
    // the rio callbacks.
    // Its a bit jank, but necessary...
    let client = unsafe {
      let mut client = Box::new(Client {
        config: config,
        connecting: true,
        nio_client: nio_client,
        rio_endpoint: None,
        unprocessed_inbound_packets: Vec::new(),
        start_time: Instant::now(),
      });
      let raw_client_ptr = Box::into_raw(client);
      let rio_endpoint = unsafe {
        util::create_rio_endpoint(raw_client_ptr, 0.0 /* time since start */)
      };
      (*raw_client_ptr).rio_endpoint = Some(rio_endpoint);
      Box::from_raw(raw_client_ptr)
    };

    client
  }

  fn tend_connection(&mut self, time_since_start_s: f64) {
    if self.rio_endpoint.is_none() {
      debug!("Tended disconnected connection");
      return;
    }

    let still_connected = unsafe {
      nio::netcode_client_state(self.nio_client) == (nio::NETCODE_CLIENT_STATE_CONNECTED as i32)
    };

    if !self.connecting && !still_connected {
      // Rio endpoint guaranteed to exist at this point
      unsafe {
        rio::reliable_endpoint_destroy(self.rio_endpoint.take().unwrap());
      }
      return;
    }

    if self.connecting && still_connected {
      self.connecting = false;
    }

    debug_assert!(self.rio_endpoint.is_some());
    self.tend_packets();
    self.tend_acks(time_since_start_s);
  }

  fn tend_packets(&self) {
    let rio_endpoint = self.rio_endpoint.unwrap();

    loop {
      let mut packet_byte_count = 0;
      let mut packet_sequence = 0;
      let received_packet = unsafe {
        nio::netcode_client_receive_packet(
          self.nio_client,
          &mut packet_byte_count,
          &mut packet_sequence,
        )
      };

      if received_packet == std::ptr::null_mut() {
        break;
      }

      // Extracts ACK metadata and processes packet via callback
      unsafe {
        rio::reliable_endpoint_receive_packet(rio_endpoint, received_packet, packet_byte_count);
        nio::netcode_client_free_packet(self.nio_client, received_packet);
      }

      // TODO(acmcarther): This duplicates logic with tend_connection: fix
      {
        let disconnected = unsafe {
          nio::netcode_client_state(self.nio_client)
            <= (nio::NETCODE_CLIENT_STATE_DISCONNECTED as i32)
        };
        if disconnected {
          break;
        }
      }
    }
  }

  fn tend_acks(&self, time_since_start_s: f64) {
    let rio_endpoint = self.rio_endpoint.unwrap();
    unsafe {
      rio::reliable_endpoint_update(rio_endpoint, time_since_start_s);
    }

    let mut ack_count = 0;

    let acks = unsafe { rio::reliable_endpoint_get_acks(rio_endpoint, &mut ack_count) };

    // TODO(acmcarther): Perform rebroadcast

    unsafe { rio::reliable_endpoint_clear_acks(rio_endpoint) };
  }
}

impl NetcodeClient for Client {
  fn is_connected(&self) -> bool {
    self.rio_endpoint.is_some()
  }

  fn send_packet(&self, mut payload_bytes: Vec<u8>) {
    if self.rio_endpoint.is_none() {
      // Client must be remade
      warn!("Tried to send a packet with a disconnected client");
      return;
    }

    unsafe {
      rio::reliable_endpoint_send_packet(
        self.rio_endpoint.unwrap(),
        payload_bytes.as_mut_ptr(),
        payload_bytes.len() as i32,
      );
    }
  }

  fn update(&mut self) {
    if self.rio_endpoint.is_none() {
      // Client must be remade
      warn!("Tried to run an update with a disconnected client");
      return;
    }

    let now = Instant::now();
    let duration = now.duration_since(self.start_time);
    let time_since_start_s =
      (duration.as_secs() as f64) + ((duration.subsec_nanos()) as f64 / 1_000_000_000.0);
    unsafe {
      nio::netcode_client_update(self.nio_client, time_since_start_s);
    }
    self.tend_connection(time_since_start_s);
  }

  fn retrieve_packets(&mut self) -> Vec<Vec<u8>> {
    let mut swap_vec = Vec::new();
    std::mem::swap(&mut self.unprocessed_inbound_packets, &mut swap_vec);
    swap_vec
  }
}

impl Drop for Client {
  fn drop(&mut self) {
    if let Some(endpoint) = self.rio_endpoint.take() {
      unsafe {
        rio::reliable_endpoint_destroy(endpoint);
      }
    }

    unsafe {
      nio::netcode_client_destroy(self.nio_client);
    }
  }
}

mod util {
  use libc;
  use Client;
  use nio;
  use rio;
  use std;
  use std::ffi::CString;
  use std::os::raw::c_void;
  use std::ptr;

  pub unsafe fn create_rio_endpoint(
    client: *mut Client,
    init_time: f64,
  ) -> *mut rio::reliable_endpoint_t {
    let mut rio_client_config: rio::reliable_config_t = std::mem::uninitialized();

    rio::reliable_default_config(&mut rio_client_config);

    rio_client_config.fragment_above = (*client).config.max_unfragmented_packet_size as i32;

    let rio_client_name = CString::new(format!("singular-client")).unwrap();
    ptr::copy(
      rio_client_name.as_ptr(),
      rio_client_config.name.as_mut_ptr(),
      rio_client_name.as_bytes_with_nul().len(),
    );

    rio_client_config.context = client as *mut c_void;
    rio_client_config.index = 0;
    rio_client_config.transmit_packet_function = Some(rio_transmit_packet_function);
    rio_client_config.process_packet_function = Some(rio_process_packet_function);

    rio::reliable_endpoint_create(&mut rio_client_config, init_time)
  }

  unsafe extern "C" fn rio_transmit_packet_function(
    context: *mut ::std::os::raw::c_void,
    slot: ::std::os::raw::c_int,
    sequence: u16,
    packet_data: *mut u8,
    packet_byte_count: ::std::os::raw::c_int,
  ) {
    if slot != 0 {
      // N.B: Currently assume that client never talks to any slot except the first one.
      error!("Client tried to emit packet to non-zero slot");
      return;
    }

    let client = context as *mut ::Client;
    assert!((*client).nio_client != std::ptr::null_mut());
    nio::netcode_client_send_packet((*client).nio_client, packet_data, packet_byte_count);
  }

  unsafe extern "C" fn rio_process_packet_function(
    context: *mut ::std::os::raw::c_void,
    slot: ::std::os::raw::c_int,
    sequence: u16,
    packet_data: *mut u8,
    packet_byte_count: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int {
    if slot != 0 {
      // N.B: Currently assume that client never talks to any slot except the first one.
      error!("Client received packet on non-zero slot, dropping");
      return 1;
    }

    let slot = slot as usize;
    let client = context as *mut ::Client;
    assert!((*client).nio_client != std::ptr::null_mut());

    let mut payload = Vec::with_capacity(packet_byte_count as usize);
    libc::memcpy(
      payload.as_mut_ptr() as *mut libc::c_void,
      packet_data as *const libc::c_void,
      packet_byte_count as usize,
    );
    payload.set_len(packet_byte_count as usize);

    (*client).unprocessed_inbound_packets.push(payload);
    1
  }
}
