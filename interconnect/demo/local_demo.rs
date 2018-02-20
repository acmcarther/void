extern crate demo_proto;
extern crate demo_state;
extern crate init;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate state_acceptor;
extern crate state_acceptor_api;
extern crate state_api;
extern crate state_proto;
extern crate state_transmitter;
extern crate state_transmitter_api;

use demo_proto::demo::ColorComponent;
use demo_proto::demo::PositionComponent;
use demo_state::COLOR_COMPONENT_ID;
use demo_state::POSITION_COMPONENT_ID;
use demo_state::State;
use protobuf::Message;
use state_acceptor::StateAcceptorConfig;
use state_acceptor::StateAcceptorImpl;
use state_acceptor_api::StateAcceptor;
use state_api::ComponentTypeId;
use state_api::EntityId;
use state_api::StateBlob;
use state_api::UpdateErr;
use state_transmitter::StateTransmitterConfig;
use state_transmitter::StateTransmitterImpl;
use state_transmitter_api::StateTransmitter;
use std::collections::HashMap;
use std::collections::HashSet;

struct Demo<'a> {
  pub client_state: State,
  pub server_state: State,
  pub client_node_id: u64,
  pub server_transmitter: &'a mut StateTransmitter,
  pub client_acceptor: &'a mut StateAcceptor,
}

fn main() {
  init::init_void();

  let mut server_transmitter = StateTransmitterImpl::from_config(StateTransmitterConfig::default());
  let mut client_acceptor = StateAcceptorImpl::from_config(StateAcceptorConfig::default());

  let mut demo = {
    let mut server_state = State::new();
    let mut client_state = State::new();

    let mut demo = Demo {
      client_state: client_state,
      server_state: server_state,
      client_node_id: 1,
      server_transmitter: &mut server_transmitter,
      client_acceptor: &mut client_acceptor,
    };
    demo.server_transmitter.add_node(demo.client_node_id);
    demo
      .server_transmitter
      .add_global_interest(&demo.client_node_id, POSITION_COMPONENT_ID);
    demo
      .server_transmitter
      .add_global_interest(&demo.client_node_id, COLOR_COMPONENT_ID);

    demo
  };

  demo.add_fancy_entity();
  demo.push_update(0.0 /* current_time_s */);

  demo.update_fancy_entity();
  demo.push_update(0.2 /* current_time_s */);

  demo.remove_fancy_color();
  demo.push_update(0.3 /* current_time_s */);
  demo.push_update(0.31 /* current_time_s */);
  demo.push_update(0.32 /* current_time_s */);

  info!("Done!");
}


impl<'a> Demo<'a> {
  pub fn add_fancy_entity(&mut self) {
    {
      let mut color_data = ColorComponent::new();
      color_data.set_r(1.0);
      color_data.set_g(0.0);
      color_data.set_b(0.0);
      color_data.set_a(1.0);
      self
        .server_state
        .update_color(1 /* entity_id */, color_data);
    }
    {
      let mut position_data = PositionComponent::new();
      position_data.set_x(1.0);
      position_data.set_y(1.0);
      position_data.set_z(1.0);
      self
        .server_state
        .update_position(1 /* entity_id */, position_data);
    }
  }


  pub fn update_fancy_entity(&mut self) {
    {
      let mut position_data = PositionComponent::new();
      position_data.set_x(-10.0);
      position_data.set_y(1.0);
      position_data.set_z(2.0);
      self
        .server_state
        .update_position(1 /* entity_id */, position_data);
    }
  }

  pub fn remove_fancy_color(&mut self) {
    self
      .server_state
      .remove_component(&1 /* entity_id */, &COLOR_COMPONENT_ID);
  }

  pub fn push_update(&mut self, current_time_s: f64) {
    info!("Server data: {:#?}", self.server_state);
    let state_update = self.server_transmitter.produce_update(
      &self.client_node_id,
      &self.server_state,
      current_time_s,
    );
    info!("State update {:#?}", state_update);
    self
      .client_acceptor
      .integrate_update(&mut self.client_state, state_update, current_time_s);
    info!("Client data: {:#?}", self.client_state);
  }
}
