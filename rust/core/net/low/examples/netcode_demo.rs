extern crate base;
#[macro_use]
extern crate log;
extern crate common;
extern crate client;
extern crate server;
extern crate netcode_io_sys as nio;

use server::NetcodeServer;
use client::NetcodeClient;
use client::Client;
use client::ClientConfig;
use server::Server;
use server::ServerConfig;

fn main() {
  base::init();
  unsafe {
    common::global::init_network();
  }

  run_demo();

  unsafe {
    common::global::term_network();
  }
  info!("Exiting");
}

fn run_demo() {
  let mut server = create_demo_server();
  let mut client_1 = create_demo_client();
  let mut client_2 = create_demo_client();

  // Make sure we have a chance to handshake before blocking on send
  server.update();
  client_1.update();
  client_2.update();

  let sleep_duration = 1.0 / 12000.0;
  for tick in 0..1000 {
    // Do server stuff
    server.update();
    for client_id in server.get_connected_clients() {
      if tick % 5 == 0 {
        debug!("server blocking start?");
        server.send_packet(&client_id, vec![0u8, 1u8, 2u8, 1u8, 0u8]);
        debug!("server blocking end?");
      }
      let client_packets = server.retrieve_packets(&client_id);
      debug!("Got {} packets from client", client_packets.len());
      for packet in client_packets {
        debug!("packet: {:#?}", packet);
      }
    }

    // Do client stuff
    {
      client_1.update();
      if tick % 4 == 0 {
        debug!("client blocking start?");
        client_1.send_packet(vec![2u8, 1u8, 0u8, 1u8, 2u8]);
        debug!("client blocking end?");
      }
      let server_packets = client_1.retrieve_packets();
      info!("Got {} packets from server", server_packets.len());
      for packet in server_packets {
        debug!("packet: {:#?}", packet);
      }
    }
    {
      client_2.update();
      if tick % 4 == 0 {
        debug!("client blocking start?");
        client_2.send_packet(vec![2u8, 1u8, 0u8, 1u8, 2u8]);
        debug!("client blocking end?");
      }
      let server_packets = client_2.retrieve_packets();
      debug!("Got {} packets from server", server_packets.len());
      for packet in server_packets {
        debug!("packet: {:#?}", packet);
      }
    }

    unsafe {
      nio::netcode_sleep(sleep_duration);
    }
  }
}

fn create_demo_server() -> Server {
  let config = ServerConfig::default();
  Server::start_from_config(config)
}

fn create_demo_client() -> Box<Client> {
  let config = ClientConfig::default();
  Client::start_from_config(config)
}
