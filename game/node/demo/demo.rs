#![feature(used)]
extern crate control_proto;
extern crate init;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate node;
extern crate physics;
extern crate physics_proto;
extern crate stores;
#[macro_use]
extern crate zcfg;

use node::DenseComponentStore;
use node::NodeConfig;
use node::ServerNode;
use node::ServiceBuilderFn;
use node::SparseComponentStore;
use physics::SimplePhysics;
use std::collections::HashMap;
use std::sync::RwLock;
use stores::StateStores;

define_cfg!(enabled_services, Vec<String>, Vec::new(), "Which services should be enabled");

pub fn main() {
  init::init();
  let config = NodeConfig {
    services: enabled_services::CONFIG.get_value(),
  };
  let mut state_stores = StateStores::new();

  seed_state_stores(&mut state_stores);

  let mut service_builders: HashMap<String, ServiceBuilderFn<StateStores>> = HashMap::new();
  service_builders.insert("simple_physics".to_owned(), SimplePhysics::new);

  let mut node = ServerNode::new(config, service_builders, state_stores);
  node.init();

  loop {
    node.tick().unwrap()
  }
}

fn seed_state_stores(state_stores: &mut StateStores) {}
