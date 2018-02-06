#![feature(used)]
extern crate init;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate netcode_io_sys as nio;
#[macro_use]
extern crate zcfg;

use init::LogLevelParsable;
use log::LogLevelFilter;
use std::ffi::CString;
use std::os::raw::c_uint;
use std::os::raw::c_void;

define_cfg!(
  nio_server_addr,
  String,
  "127.0.0.1:40000".to_owned(),
  "ADDRESS:PORT for the netcode io server"
);

define_cfg!(
  max_connection_count,
  i32,
  16,
  "Max number of accepted connections"
);


// The name is a lie, this is copied from the example
// TODO(acmcarther): Fix
mod HIGHLY_CREDIBLE {
  pub const SECRET_KEY: [u8; 32] = [
    0x60,
    0x6a,
    0xbe,
    0x6e,
    0xc9,
    0x19,
    0x10,
    0xea,
    0x9a,
    0x65,
    0x62,
    0xf6,
    0x6f,
    0x2b,
    0x30,
    0xe4,
    0x43,
    0x71,
    0xd6,
    0x2c,
    0xd1,
    0x99,
    0x27,
    0x26,
    0x6b,
    0x3c,
    0x60,
    0xf4,
    0xb7,
    0x15,
    0xab,
    0xa1,
  ];

  pub const PROTOCOL_ID: u64 = 0x1122334455667788;
}

fn find_nio_log_level(log_level_filter: LogLevelFilter) -> c_uint {
  match log_level_filter {
    LogLevelFilter::Off => nio::NETCODE_LOG_LEVEL_NONE,
    LogLevelFilter::Error | LogLevelFilter::Warn => nio::NETCODE_LOG_LEVEL_ERROR,
    LogLevelFilter::Info => nio::NETCODE_LOG_LEVEL_INFO,
    LogLevelFilter::Debug | LogLevelFilter::Trace => nio::NETCODE_LOG_LEVEL_DEBUG,
  }
}

fn main() {
  init::init_void();

  unsafe {
    if nio::netcode_init() != (nio::NETCODE_OK as i32) {
      error!("Could not initialize netcode_io");
      panic!("Could not initialize netcode_io");
    }
  }

  let LogLevelParsable(log_level) = init::log_level::CONFIG.get_value();
  let nio_log_level = find_nio_log_level(log_level);

  unsafe {
    nio::netcode_log_level(nio_log_level as i32);
  }

  let mut nio_server_config: nio::netcode_server_config_t = unsafe { std::mem::uninitialized() };


  unsafe {
    nio::netcode_default_server_config(&mut nio_server_config);
  }

  nio_server_config.protocol_id = HIGHLY_CREDIBLE::PROTOCOL_ID;
  nio_server_config.private_key = HIGHLY_CREDIBLE::SECRET_KEY;

  let addr = CString::new(nio_server_addr::CONFIG.get_value()).unwrap();
  let server: *mut nio::netcode_server_t =
    unsafe { nio::netcode_server_create(addr.as_ptr(), &nio_server_config, 0.0) };

  if server == std::ptr::null_mut() {
    error!("Could not create netcode_io server");
    panic!("Could not create netcode_io server");
  }

  unsafe {
    nio::netcode_server_start(server, max_connection_count::CONFIG.get_value());
  }

  let mut packet_data: [u8; nio::NETCODE_MAX_PACKET_SIZE as usize] =
    [0; nio::NETCODE_MAX_PACKET_SIZE as usize];
  for i in 0..nio::NETCODE_MAX_PACKET_SIZE {
    packet_data[i as usize] = i as u8;
  }

  let mut time = 0.0;
  let sleep_duration = 1.0 / 60.0;
  loop {
    unsafe {
      nio::netcode_server_update(server, time);
    }

    let client_connected = unsafe { nio::netcode_server_client_connected(server, 0) };
    if client_connected != 0 {
      unsafe {
        nio::netcode_server_send_packet(
          server,
          0,
          packet_data.as_ptr(),
          nio::NETCODE_MAX_PACKET_SIZE as i32,
        );
      }
    }

    for i in 0..nio::NETCODE_MAX_CLIENTS {
      loop {
        let mut packet_byte_count = 0;
        let mut packet_sequence = 0;
        let received_packet = unsafe {
          nio::netcode_server_receive_packet(
            server,
            i as i32,
            &mut packet_byte_count,
            &mut packet_sequence,
          )
        };

        if received_packet == std::ptr::null_mut() {
          break;
        }

        assert!(packet_byte_count == nio::NETCODE_MAX_PACKET_SIZE as i32);
        assert!(unsafe {
          libc::memcmp(
            received_packet as *const libc::c_void,
            packet_data.as_ptr() as *const libc::c_void,
            nio::NETCODE_MAX_PACKET_SIZE as usize,
          ) == 0
        });
        unsafe {
          nio::netcode_server_free_packet(server, received_packet as *mut c_void);
        }
      }
    }

    unsafe {
      nio::netcode_sleep(sleep_duration);
    }
    time = time + sleep_duration;
  }

  unsafe {
    nio::netcode_server_destroy(server);
    nio::netcode_term();
  }
}
