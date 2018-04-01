#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
  pub pos: [f32; 3],
  pub norm: [f32; 3],
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
