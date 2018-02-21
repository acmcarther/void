#![feature(used)]
extern crate demo_proto;
extern crate demo_state;
extern crate init;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate netcode_api;
extern crate netcode_client;
extern crate netcode_global;
extern crate netcode_io_sys as nio;
extern crate netcode_server;
extern crate protobuf;
extern crate state_acceptor;
extern crate state_acceptor_api;
extern crate state_api;
extern crate state_proto;
extern crate state_transmitter;
extern crate state_transmitter_api;
#[macro_use]
extern crate zcfg;

pub mod flags {
  define_pub_cfg!(
    demo_remote_role,
    String,
    "server".to_owned(),
    "Which role to take (client|server)"
  );
}

fn main() {
  init::init();
  unsafe {
    netcode_global::init_network();
  }

  match flags::demo_remote_role::CONFIG
    .get_value()
    .to_lowercase()
    .as_ref()
  {
    "server" => server::be_a_server(),
    "client" => client::be_a_client(),
    something_else => error!(
      "Unknown remote demo role {} (try server or client)",
      something_else
    ),
  }

  unsafe {
    netcode_global::term_network();
  }
}


mod server {
  use demo_proto::demo::ColorComponent;
  use demo_proto::demo::PositionComponent;
  use demo_proto::demo::ServerMessage;
  use demo_state::COLOR_COMPONENT_ID;
  use demo_state::POSITION_COMPONENT_ID;
  use demo_state::State;
  use netcode_api::ClientId;
  use netcode_api::NetcodeServer;
  use netcode_server::Server;
  use netcode_server::ServerConfig;
  use nio;
  use protobuf::Message;
  use state_transmitter::ComponentConfig;
  use state_transmitter::StateTransmitterConfig;
  use state_transmitter::StateTransmitterImpl;
  use state_transmitter_api::StateTransmitter;
  use std::collections::HashSet;
  use std::time::Instant;

  struct DemoServer {
    server: Server,
    state: State,
    client_ids: HashSet<ClientId>,
    state_transmitter: StateTransmitterImpl,
    start_time: Instant,
  }

  pub fn be_a_server() {
    info!("I'll be a server!");
    let server = Server::start_from_config(ServerConfig::default());
    let mut state_transmitter_config = StateTransmitterConfig::default();
    {
      state_transmitter_config.component_configs.insert(
        POSITION_COMPONENT_ID,
        ComponentConfig {
          s_between_frames: 0.01,
          delta_frames_per_keyframe: 0,
        },
      );
      state_transmitter_config.component_configs.insert(
        COLOR_COMPONENT_ID,
        ComponentConfig {
          s_between_frames: 3.00,
          delta_frames_per_keyframe: 0,
        },
      );
    }
    let state_transmitter = StateTransmitterImpl::from_config(state_transmitter_config);

    let mut demo_server = DemoServer {
      server: server,
      client_ids: HashSet::new(),
      state_transmitter: state_transmitter,
      state: State::new(),
      start_time: Instant::now(),
    };

    {
      {
        let mut color_data = ColorComponent::new();
        color_data.set_r(1.0);
        color_data.set_g(0.0);
        color_data.set_b(0.0);
        color_data.set_a(1.0);
        demo_server
          .state
          .update_color(1 /* entity_id */, color_data);
      }
      {
        let mut position_data = PositionComponent::new();
        position_data.set_x(1.0);
        position_data.set_y(1.0);
        position_data.set_z(1.0);
        demo_server
          .state
          .update_position(1 /* entity_id */, position_data);
      }
    }

    let sleep_duration = 1.0 / 12000.0;

    loop {
      let current_time_s = {
        let now = Instant::now();
        let duration = now.duration_since(demo_server.start_time);
        (duration.as_secs() as f64) + ((duration.subsec_nanos()) as f64 / 1_000_000_000.0)
      };

      {
        demo_server
          .state
          .mut_position(&1 /* entity_id */)
          .unwrap()
          .set_x(current_time_s as f32);
        demo_server
          .state
          .mut_color(&1 /* entity_id */)
          .unwrap()
          .set_r((current_time_s as f32).fract());
      }

      demo_server.server.update();

      let mut removed_client_ids = demo_server.client_ids.clone();

      for client_id in demo_server.server.get_connected_clients() {
        removed_client_ids.remove(&client_id);
        let node_id = client_id as u64;

        if !demo_server.client_ids.contains(&client_id) {
          demo_server.client_ids.insert(client_id.clone());
          info!("Registering node_id {}", node_id);
          demo_server.state_transmitter.add_node(node_id);
          demo_server
            .state_transmitter
            .add_global_interest(&node_id, POSITION_COMPONENT_ID);
          demo_server
            .state_transmitter
            .add_global_interest(&node_id, COLOR_COMPONENT_ID);
        }

        let state_update = demo_server.state_transmitter.produce_update(
          &node_id,
          &demo_server.state,
          current_time_s,
        );
        if state_update.get_component_updates().len() > 0 {
          info!("sending nice payload, {:#?}", state_update);
          let update_bytes = state_update.write_to_bytes().unwrap();
          let mut server_message = ServerMessage::new();
          server_message.set_data(update_bytes);
          let message_bytes = server_message.write_to_bytes().unwrap();
          demo_server.server.send_packet(&client_id, message_bytes);
        } else {
        }

        // Dump client packets
        demo_server.server.retrieve_packets(&client_id);
      }

      for removed_client_id in removed_client_ids.into_iter() {
        debug!(
          "removing client {} from state_transmitter",
          removed_client_id
        );
        demo_server
          .state_transmitter
          .remove_node(&(removed_client_id as u64));
      }

      //info!("Server data: {:#?}", demo_server.state);

      unsafe {
        nio::netcode_sleep(sleep_duration);
      }
    }
  }
}

mod client {
  use demo_proto::demo::ServerMessage;
  use demo_state::State;
  use netcode_api::NetcodeClient;
  use netcode_client::Client;
  use netcode_client::ClientConfig;
  use nio;
  use protobuf;
  use state_acceptor::StateAcceptorConfig;
  use state_acceptor::StateAcceptorImpl;
  use state_acceptor_api::StateAcceptor;
  use state_proto::state::StateUpdate;
  use std::time::Instant;

  struct DemoClient {
    client: Box<Client>,
    state: State,
    state_acceptor: StateAcceptorImpl,
    start_time: Instant,
  }

  pub fn be_a_client() {
    info!("I'll be a client!");
    let config = ClientConfig::default();
    let client = Client::start_from_config(config);

    let mut demo_client = DemoClient {
      client: client,
      start_time: Instant::now(),
      state: State::new(),
      state_acceptor: StateAcceptorImpl::from_config(StateAcceptorConfig::default()),
    };
    let sleep_duration = 1.0 / 12000.0;

    loop {
      let current_time_s = {
        let now = Instant::now();
        let duration = now.duration_since(demo_client.start_time);
        (duration.as_secs() as f64) + ((duration.subsec_nanos()) as f64 / 1_000_000_000.0)
      };

      demo_client.client.update();

      if demo_client.client.is_connected() {
        for server_packet in demo_client.client.retrieve_packets().into_iter() {
          let msg_result = protobuf::parse_from_bytes::<ServerMessage>(server_packet.as_slice());
          if msg_result.is_err() {
            error!("Could not parse payload from server!");
            continue;
          }

          let mut server_message = msg_result.unwrap();

          let update_result = protobuf::parse_from_bytes::<StateUpdate>(server_message.get_data());
          if update_result.is_err() {
            info!("Could not parse data payload from server");
            continue;
          }

          let update = update_result.unwrap();

          demo_client.state_acceptor.integrate_update(
            &mut demo_client.state,
            update,
            current_time_s,
          );
          info!("Client data: {:#?}", demo_client.state);
        }
      }

      unsafe {
        nio::netcode_sleep(sleep_duration);
      }
    }
  }
}
