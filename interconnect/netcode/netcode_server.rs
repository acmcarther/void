#![feature(used)]
extern crate init;
extern crate libc;
#[macro_use]
extern crate log;
extern crate netcode_api;
extern crate netcode_io_sys as nio;
extern crate netcode_terrible_constants;
extern crate reliable_io_sys as rio;

use netcode_api::ClientId;
use netcode_api::ClientSlot;
use netcode_api::NetcodeServer;
use netcode_api::flags as netcode_api_flags;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;
use std::time::Instant;

#[derive(Clone)]
pub struct ServerConfig {
  pub private_key: [u8; 32],
  pub protocol_id: u64,
  pub server_addr: String,
  pub max_connection_count: u32,
  pub max_unfragmented_packet_size: u32,
}

pub struct ClientDetails {
  pub client_id: ClientId,
  unprocessed_inbound_packets: Vec<Vec<u8>>,
  client_slot: Option<ClientSlot>,
  rio_endpoint: Option<*mut rio::reliable_endpoint_t>,
}

struct ServerContext {
  nio_server: *mut nio::netcode_server_t,
  slot_to_client_id: HashMap<ClientSlot, ClientId>,
  client_details: HashMap<ClientId, ClientDetails>,
}

pub struct Server {
  config: ServerConfig,
  context: Box<ServerContext>,
  // This is the same as the above box, used to create rio_endpoint_t instances
  // It needs to be here redundantly because getting a raw pointer from a box consumes the box.
  // Idiotic.
  context_ptr: *mut ServerContext,
  start_time: Instant,
}

impl Default for ServerConfig {
  fn default() -> ServerConfig {
    ServerConfig {
      private_key: netcode_terrible_constants::PRIVATE_KEY,
      protocol_id: netcode_terrible_constants::PROTOCOL_ID,
      server_addr: netcode_api_flags::nio_server_addr::CONFIG.get_value(),
      max_connection_count: netcode_api_flags::max_connection_count::CONFIG.get_value(),
      max_unfragmented_packet_size: netcode_api_flags::max_unfragmented_packet_size::CONFIG
        .get_value(),
    }
  }
}

impl ClientDetails {
  unsafe fn new_connected(
    client_id: ClientId,
    client_slot: ClientSlot,
    rio_endpoint: *mut rio::reliable_endpoint_t,
  ) -> ClientDetails {
    ClientDetails {
      client_id: client_id,
      client_slot: Some(client_slot),
      unprocessed_inbound_packets: Vec::new(),
      rio_endpoint: Some(rio_endpoint),
    }
  }

  fn connected(&self) -> bool {
    self.client_slot.is_some()
  }

  fn disconnect(&mut self) {
    debug!("Setting client {} to disconnected", self.client_id);
    if self.client_slot.is_none() {
      return;
    }

    self.client_slot = None;
    if self.unprocessed_inbound_packets.len() > 0 {
      // This is anomalous: applications should structure themselves such that they handle payloads
      // before doing live checks
      warn!(
        "Client {} just disconnected with {} inbound payloads pending. Payloads discarded.",
        self.client_id,
        self.unprocessed_inbound_packets.len()
      );
      self.unprocessed_inbound_packets.drain(..);
    }

    unsafe {
      rio::reliable_endpoint_destroy(self.rio_endpoint.take().unwrap());
    }
  }

  fn reconnect(&mut self, client_slot: ClientSlot, rio_endpoint: *mut rio::reliable_endpoint_t) {
    assert!(self.client_slot.is_none());

    self.client_slot = Some(client_slot);
    self.rio_endpoint = Some(rio_endpoint);
  }
}

impl Drop for ClientDetails {
  fn drop(&mut self) {
    if self.rio_endpoint.is_some() {
      unsafe {
        rio::reliable_endpoint_destroy(self.rio_endpoint.unwrap());
      }
    }
  }
}

impl ServerContext {
  fn tend_packets(&mut self, client_slot: ClientSlot) {
    let client_id = self.slot_to_client_id.get(&client_slot).unwrap().clone();
    assert!(self.client_details.contains_key(&client_id));
    let client_details = self.client_details.get_mut(&client_id).unwrap();
    let rio_endpoint = client_details.rio_endpoint.unwrap();
    loop {
      let mut packet_byte_count = 0;
      let mut packet_sequence = 0;
      let received_packet = unsafe {
        nio::netcode_server_receive_packet(
          self.nio_server,
          client_slot as i32,
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
        nio::netcode_server_free_packet(self.nio_server, received_packet as *mut c_void);
      }
    }
  }

  fn tend_acks(&mut self, client_slot: ClientSlot, time_since_start_s: f64) {
    let client_id = self.slot_to_client_id.get(&client_slot).unwrap();
    assert!(self.client_details.contains_key(client_id));
    let client_details = self.client_details.get_mut(client_id).unwrap();
    let rio_endpoint = client_details.rio_endpoint.unwrap();
    unsafe {
      rio::reliable_endpoint_update(rio_endpoint, time_since_start_s);
    }

    let mut ack_count = 0;

    let acks = unsafe { rio::reliable_endpoint_get_acks(rio_endpoint, &mut ack_count) };

    // TODO(acmcarther): Perform rebroadcast

    unsafe { rio::reliable_endpoint_clear_acks(rio_endpoint) };
  }
}

impl Server {
  pub fn start_from_config(config: ServerConfig) -> Server {
    let nio_server = {
      let mut nio_server_config: nio::netcode_server_config_t =
        unsafe { std::mem::uninitialized() };

      unsafe {
        nio::netcode_default_server_config(&mut nio_server_config);
      }

      nio_server_config.protocol_id = config.protocol_id;
      nio_server_config.private_key = config.private_key.clone();

      let addr = CString::new(config.server_addr.clone()).unwrap();
      let nio_server: *mut nio::netcode_server_t =
        unsafe { nio::netcode_server_create(addr.as_ptr(), &nio_server_config, 0.0) };

      if nio_server == std::ptr::null_mut() {
        error!("Could not create netcode_io server");
        panic!("Could not create netcode_io server");
      }

      unsafe {
        nio::netcode_server_start(nio_server, config.max_connection_count as i32);
      }

      nio_server
    };

    let (context, context_ptr) = unsafe {
      // Perform a silly box dance because Box::into_raw consumes the box
      let context = Box::new(ServerContext {
        nio_server: nio_server,
        slot_to_client_id: HashMap::new(),
        client_details: HashMap::new(),
      });
      let context_ptr = Box::into_raw(context);
      let context = Box::from_raw(context_ptr);

      (context, context_ptr)
    };

    Server {
      config: config.clone(),
      context: context,
      context_ptr: context_ptr,
      start_time: Instant::now(),
    }
  }

  fn tend_connections(&mut self, time_since_start_s: f64) {
    for client_slot in 0..self.config.max_connection_count {
      let client_slot = client_slot as ClientSlot;

      let slot_connected = self.tend_client_slot(client_slot, time_since_start_s);
      if !slot_connected {
        continue;
      }

      self.context.tend_packets(client_slot);
      self.context.tend_acks(client_slot, time_since_start_s);
    }
  }

  fn tend_client_slot(&mut self, client_slot: ClientSlot, time_since_start_s: f64) -> bool {
    let slot_connected = unsafe {
      nio::netcode_server_client_connected(self.context.nio_server, client_slot as i32) > 0
    };

    if !slot_connected {
      if self.context.slot_to_client_id.contains_key(&client_slot) {
        let client_id = self.context.slot_to_client_id.remove(&client_slot).unwrap();
        assert!(self.context.client_details.contains_key(&client_id));
        self
          .context
          .client_details
          .get_mut(&client_id)
          .unwrap()
          .disconnect();
      }
      return false; /* slot not connected */
    }

    let client_id = unsafe {
      nio::netcode_server_client_id(self.context.nio_server, client_slot as i32) as ClientId
    };
    let has_existing_client = self.context.slot_to_client_id.contains_key(&client_slot);
    let existing_client_equals_current = self
      .context
      .slot_to_client_id
      .get(&client_slot)
      .map(|c_id| *c_id == client_id)
      .unwrap_or(false);

    if has_existing_client && existing_client_equals_current {
      // No work to do, connection is up to date
      return true /* slot is connected */;
    }

    // Prune hijacked slot's old connection
    if has_existing_client
    /* && existing client isnt current */
    {
      debug!("Connection in slot {} was hijacked", client_slot);
      let old_client_id = self.context.slot_to_client_id.remove(&client_slot).unwrap();
      assert!(self.context.client_details.contains_key(&old_client_id));
      self
        .context
        .client_details
        .get_mut(&old_client_id)
        .unwrap()
        .disconnect();
    }

    // Reconnect or create client
    let rio_endpoint =
      unsafe { util::create_rio_endpoint(self, client_slot, client_id, time_since_start_s) };
    if self.context.client_details.contains_key(&client_id) {
      self
        .context
        .client_details
        .get_mut(&client_id)
        .unwrap()
        .reconnect(client_slot, rio_endpoint)
    } else {
      self.context.client_details.insert(client_id, unsafe {
        ClientDetails::new_connected(client_id, client_slot, rio_endpoint)
      });
    }
    self
      .context
      .slot_to_client_id
      .insert(client_slot, client_id);

    return true /* slot is connected */;
  }
}

impl NetcodeServer for Server {
  fn get_connected_clients(&self) -> Vec<ClientId> {
    self.context.slot_to_client_id.values().cloned().collect()
  }

  fn send_packet(&mut self, client_id: &ClientId, mut payload_bytes: Vec<u8>) {
    if !self.context.client_details.contains_key(client_id) {
      warn!(
        "Tried to send a packet to totally unknown client {}: it was dropped instead",
        client_id
      );
      return;
    }

    let client_details = self.context.client_details.get_mut(client_id).unwrap();
    if client_details.client_slot.is_none() {
      warn!(
        "Tried to send a packet to a disconnected client {}: it was dropped instead",
        client_id
      );
      return;
    }

    unsafe {
      rio::reliable_endpoint_send_packet(
        client_details.rio_endpoint.unwrap(),
        payload_bytes.as_mut_ptr(),
        payload_bytes.len() as i32,
      );
    }
  }

  fn update(&mut self) {
    let now = Instant::now();
    let duration = now.duration_since(self.start_time);
    let time_since_start_s =
      (duration.as_secs() as f64) + ((duration.subsec_nanos()) as f64 / 1_000_000_000.0);
    unsafe {
      nio::netcode_server_update(self.context.nio_server, time_since_start_s);
    }
    self.tend_connections(time_since_start_s);
  }

  /** Fetches packets to be processed for a client. */
  fn retrieve_packets(&mut self, client_id: &ClientId) -> Vec<Vec<u8>> {
    if !self.context.client_details.contains_key(client_id) {
      warn!("Tried to retrieve packets for unknown client {}", client_id);
      return Vec::new();
    }

    let mut swap_vec = Vec::new();
    std::mem::swap(
      &mut self
        .context
        .client_details
        .get_mut(client_id)
        .unwrap()
        .unprocessed_inbound_packets,
      &mut swap_vec,
    );
    swap_vec
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    unsafe {
      nio::netcode_server_destroy(self.context.nio_server);
    }
  }
}

mod util {
  use Server;
  use libc;
  use netcode_api::ClientId;
  use netcode_api::ClientSlot;
  use nio;
  use rio;
  use std;
  use std::ffi::CString;
  use std::os::raw::c_void;
  use std::ptr;

  pub unsafe fn create_rio_endpoint(
    server: &mut Server,
    client_slot: ClientSlot,
    client_id: ClientId,
    init_time: f64,
  ) -> *mut rio::reliable_endpoint_t {
    let mut rio_server_config: rio::reliable_config_t = std::mem::uninitialized();

    rio::reliable_default_config(&mut rio_server_config);

    rio_server_config.fragment_above = server.config.max_unfragmented_packet_size as i32;

    let rio_server_name = CString::new(format!("client-{}@{}", client_id, client_slot)).unwrap();
    ptr::copy(
      rio_server_name.as_ptr(),
      rio_server_config.name.as_mut_ptr(),
      rio_server_name.as_bytes_with_nul().len(),
    );

    rio_server_config.context = server.context_ptr as *mut c_void;
    rio_server_config.index = client_slot as i32;
    rio_server_config.transmit_packet_function = Some(rio_transmit_packet_function);
    rio_server_config.process_packet_function = Some(rio_process_packet_function);

    rio::reliable_endpoint_create(&mut rio_server_config, init_time)
  }

  unsafe extern "C" fn rio_transmit_packet_function(
    context: *mut ::std::os::raw::c_void,
    slot: ::std::os::raw::c_int,
    sequence: u16,
    packet_data: *mut u8,
    packet_byte_count: ::std::os::raw::c_int,
  ) {
    let server_context = context as *mut ::ServerContext;
    assert!((*server_context).nio_server != std::ptr::null_mut());
    nio::netcode_server_send_packet(
      (*server_context).nio_server,
      slot,
      packet_data,
      packet_byte_count,
    );
  }

  unsafe extern "C" fn rio_process_packet_function(
    context: *mut ::std::os::raw::c_void,
    slot: ::std::os::raw::c_int,
    sequence: u16,
    packet_data: *mut u8,
    packet_byte_count: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int {
    let slot = slot as usize;
    let server_context = context as *mut ::ServerContext;

    let mut payload = Vec::with_capacity(packet_byte_count as usize);
    libc::memcpy(
      payload.as_mut_ptr() as *mut libc::c_void,
      packet_data as *const libc::c_void,
      packet_byte_count as usize,
    );

    assert!((*server_context).slot_to_client_id.contains_key(&slot));
    let client_id = (*server_context).slot_to_client_id.get(&slot).unwrap();
    assert!((*server_context).client_details.contains_key(client_id));
    let client_details = (*server_context).client_details.get_mut(client_id).unwrap();

    client_details.unprocessed_inbound_packets.push(payload);
    1
  }
}
