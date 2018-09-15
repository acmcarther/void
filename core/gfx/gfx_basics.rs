extern crate vector;

use vector::Vector3;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
  pub pos: Vector3<f32>,
  pub norm: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl Mesh {
  pub fn num_faces(&self) -> usize {
    self.indices.len() / 3
  }
}
