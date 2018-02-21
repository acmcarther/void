extern crate base_server;
extern crate control_proto;
extern crate physics_proto;

use base_server::DenseComponentStore;
use base_server::SparseComponentStore;
use control_proto::control::ControlledInfo;
use control_proto::control::ControllerInfo;
use physics_proto::physics;
use physics_proto::physics::PhysicsInfo;
use std::sync::RwLock;

pub struct StateStores {
  pub physics_store: RwLock<DenseComponentStore<PhysicsInfo>>,
  pub impulse_store: RwLock<DenseComponentStore<physics::Vec3>>,
  pub controlled_store: RwLock<SparseComponentStore<ControlledInfo>>,
  pub controller_store: RwLock<SparseComponentStore<ControllerInfo>>,
}

impl StateStores {
  pub fn new() -> StateStores {
    StateStores {
      physics_store: RwLock::new(DenseComponentStore::new()),
      impulse_store: RwLock::new(DenseComponentStore::new()),
      controlled_store: RwLock::new(SparseComponentStore::new()),
      controller_store: RwLock::new(SparseComponentStore::new()),
    }
  }
}
