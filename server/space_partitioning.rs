pub enum BHNode {
  EmptyNode,
  InternalNode {
    center_of_mass: (i64, i64, i64),
    total_mass: i64
    children: [Box<BHTree>;8]
  }
  ExternalNode {
    body_position: (i64, i64, i64),
    body_mass: i64
  }
}

impl BHTree {
  pub fn new() -> BHTree {
    BHTree::EmptyNode,
  }

  pub fn insert(&mut self) -> BHTree {
    match &self {
    }
  }
}
