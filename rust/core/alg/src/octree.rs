use std::default::Default;
use std::iter;
use std::mem;
use std::ops::DerefMut;

const X_INDEX: usize = 0;
const Y_INDEX: usize = 1;
const Z_INDEX: usize = 2;

pub trait AsCoord {
  fn get_coord(&self) -> &[f32; 3];
}

impl AsCoord for [f32; 3] {
  fn get_coord(&self) -> &[f32; 3] {
    &self
  }
}

pub trait UpdateCoord {
  fn update_coord(&mut self, to: [f32; 3]);
}

pub trait Aggregate {
  fn combine(&self, other: &Self) -> Self;
}

#[derive(Clone, Debug)]
pub struct OctreeInitParams {
  pub node_capacity: usize,
  pub center: [f32; 3],
  pub half_size: [f32; 3],
  // Below what percentage of node occupancy should a merge occur
  pub desired_node_occupancy_ratio_min: f32,
  // Enabling root resizing causes the initial size settings to not be very relevant, as the
  // process of populating the tree from zero will probably dramatically shrink the tree.
  pub resize_tree_bounds: bool,
  // Below what percentage of in-tree occupancy should a tree-grow occur
  // N.B.: Setting this to 0.999 will effectively force upscaling
  // N.B.: Setting this to 0.0 will effectively disable upscaling
  pub desired_tree_occupancy_ratio_min: f32,
  // Above what percentage of in-tree occupancy should a tree-shrink occur
  // N.B.: Setting this to 1.0 will effectively disable downscaling
  // N.B.: Setting this to 0.0 will trend the tree toward containing none of the affected points
  pub desired_tree_occupancy_ratio_max: f32,
  // Mimimum tree population before considering growing or shrinking the tree on insert/remove
  pub tree_resize_minimum_population: usize,
}

#[derive(Clone, Debug)]
struct OctreeConfig {
  node_capacity: usize,
  // This number is important to tune. Too large of a value will cause removals to become expensive
  // (as we need to count nodes a lot to perform a removal). Too small a value will leave
  // suboptimal splits for longer.
  desired_node_occupancy_ratio_min: f32,
}

#[derive(Debug)]
pub struct OctreeRootNode<D, M = ()> {
  params: OctreeInitParams,
  out_of_volume_data: Vec<D>,
  inner_node: OctreeNode<D, M>,
  out_of_volume_metadata: M,
}

#[derive(Debug)]
struct OctreeNode<D, M = ()> {
  config: OctreeConfig,
  data: Vec<D>,
  center: [f32; 3],
  half_size: [f32; 3],
  is_leaf: bool,
  population: usize,
  children: [Option<Box<OctreeNode<D, M>>>; 8],
  metadata: M,
}

struct NodeBounds {
  center: [f32; 3],
  half_size: [f32; 3],
}

struct RemovalDetails<D> {
  data: D,
  should_consider_merging: bool,
  merging_current_depth: usize,
}

pub struct NodeTraversalData<'a, D: 'a, M: 'a> {
  pub data: &'a Vec<D>,
  pub metadata: &'a M,
  pub center: &'a [f32; 3],
  pub half_size: &'a [f32; 3],
  pub is_leaf: bool,
  // Indicates that the above `center` and `size` specify the volume not captured by this space.
  // This is useful for modeling out-of-volume data.
  pub negate_specified_bouunds: bool,
}

pub struct ForwardTraversalResult<TO> {
  pub should_continue: bool,
  pub partial_result: TO,
}

impl Default for OctreeInitParams {
  fn default() -> OctreeInitParams {
    OctreeInitParams {
      node_capacity: 8,
      center: [0f32; 3],
      half_size: [100f32; 3],
      desired_node_occupancy_ratio_min: 0.376,
      resize_tree_bounds: true,
      desired_tree_occupancy_ratio_min: 0.95,
      desired_tree_occupancy_ratio_max: 0.98,
      tree_resize_minimum_population: 50,
    }
  }
}

impl<D: AsCoord, M: Default> OctreeRootNode<D, M> {
  pub fn new(params: OctreeInitParams) -> OctreeRootNode<D, M> {
    OctreeRootNode {
      out_of_volume_data: Vec::new(),
      out_of_volume_metadata: M::default(),
      inner_node: OctreeNode {
        config: OctreeConfig {
          node_capacity: params.node_capacity.clone(),
          desired_node_occupancy_ratio_min: params.desired_node_occupancy_ratio_min.clone(),
        },
        data: Vec::new(),
        center: params.center.clone(),
        half_size: params.half_size.clone(),
        is_leaf: true,
        children: [None, None, None, None, None, None, None, None],
        population: 0,
        metadata: M::default(),
      },
      params: params,
    }
  }

  pub fn insert(&mut self, item: D) {
    let is_out_of_volume = self.inner_node.coord_is_out_of_bounds(item.get_coord());

    if !is_out_of_volume {
      self.inner_node.insert(item);
      return;
    }

    let already_exists_out_of_volume = {
      let coord = item.get_coord();
      self
        .out_of_volume_data
        .iter()
        .any(|e| e.get_coord() == coord)
    };

    if already_exists_out_of_volume {
      return;
    }

    self.out_of_volume_data.push(item);
    if self.params.resize_tree_bounds && self.len() > self.params.tree_resize_minimum_population {
      self.try_to_grow_whole_tree();
    }
  }

  fn try_to_grow_whole_tree(&mut self) {
    debug!("Trying to resize tree");
    if self.out_of_volume_data.is_empty() {
      // Don't know how to grow if there are no out of tree nodes.
      return;
    }

    let mut in_tree_occupancy_ratio = 1f32 - (self.out_of_volume_len() as f32 / self.len() as f32);

    debug!("occupancy ratio is {}", in_tree_occupancy_ratio);
    while in_tree_occupancy_ratio < 1.0
      && in_tree_occupancy_ratio < self.params.desired_tree_occupancy_ratio_min
    {
      let mut out_of_volume_population_per_octant = [0; 8];

      for entry in self.out_of_volume_data.iter() {
        let octant_idx = self
          .inner_node
          .find_child_index_for_coord(entry.get_coord());
        out_of_volume_population_per_octant[octant_idx] += 1;
      }

      // UNWRAP: Iterated object is of fixed size
      let (max_idx, _) = out_of_volume_population_per_octant
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .unwrap();

      let double_size = [
        self.inner_node.half_size[X_INDEX] * 2.0,
        self.inner_node.half_size[Y_INDEX] * 2.0,
        self.inner_node.half_size[Z_INDEX] * 2.0,
      ];

      let parent_direction_relative_to_child = {
        // The darkness won...
        [
          (((max_idx >> 2 & 1) * 2) as f32 - 1f32),
          (((max_idx >> 1 & 1) * 2) as f32 - 1f32),
          (((max_idx & 1) * 2) as f32 - 1f32),
        ]
      };

      let new_center = {
        let x_offset =
          -(parent_direction_relative_to_child[X_INDEX] * self.inner_node.half_size[X_INDEX]);
        let y_offset =
          -(parent_direction_relative_to_child[Y_INDEX] * self.inner_node.half_size[Y_INDEX]);
        let z_offset =
          -(parent_direction_relative_to_child[Z_INDEX] * self.inner_node.half_size[Z_INDEX]);
        // Parent center is current center *minus* offset relative to child
        let x_center = self.inner_node.center[X_INDEX] - x_offset;
        let y_center = self.inner_node.center[Y_INDEX] - y_offset;
        let z_center = self.inner_node.center[Z_INDEX] - z_offset;
        [x_center, y_center, z_center]
      };

      unsafe {
        // Take ownership of inner node without initializing a replacement
        let mut inner_node = mem::uninitialized();
        mem::swap(&mut inner_node, &mut self.inner_node);

        let mut new_outer_node = OctreeNode {
          config: inner_node.config.clone(),
          data: Vec::new(),
          center: new_center.clone(),
          // This looks confusing, but see `OctreeNode::insert` to understand how this is similar to
          // (but opposite from) insertion
          half_size: double_size.clone(),
          is_leaf: true,
          children: [None, None, None, None, None, None, None, None],
          population: 0,
          metadata: M::default(),
        };

        new_outer_node.population = inner_node.population;
        new_outer_node.is_leaf = false;

        let current_root_new_child_idx =
          new_outer_node.find_child_index_for_coord(&inner_node.center);

        // Replace the empty node (where our former inner_node belongs)
        // ... but dont `forget` it, because it is a structurally complete node and needs to be
        // dropped.
        let mut boxed_inner_node = Some(Box::new(inner_node));
        // UNWRAP: Produced as `some`
        if boxed_inner_node.as_ref().unwrap().len() != 0 {
          mem::swap(
            // UNWRAP: Known to be populated from above code
            &mut new_outer_node.children[current_root_new_child_idx],
            &mut boxed_inner_node,
          );
        }

        // Replace the currently uninitialized inner_node with this new outer node
        // ... and forget about the uninitialized value that we are left with.
        mem::swap(&mut new_outer_node, &mut self.inner_node);
        mem::forget(new_outer_node)
      }

      // Try to move some of the out of volume data into the volume
      {
        let mut new_out_of_volume_data = Vec::new();
        for entry in self.out_of_volume_data.drain(..) {
          let is_out_of_volume = self.inner_node.coord_is_out_of_bounds(entry.get_coord());

          if !is_out_of_volume {
            self.inner_node.insert(entry);
          } else {
            new_out_of_volume_data.push(entry);
          }
        }
        mem::swap(&mut new_out_of_volume_data, &mut self.out_of_volume_data);
      }

      // Update the occupancy ratio so that we can see if we need to enlarge again.  This may
      // happen if most of the out of tree nodes are concentrated very far away from the octree
      // (multiples of the current width).
      in_tree_occupancy_ratio = 1f32 - (self.out_of_volume_len() as f32 / self.len() as f32);
    }
  }
}

impl<D: AsCoord, M> OctreeRootNode<D, M> {
  pub fn out_of_volume_len(&self) -> usize {
    self.out_of_volume_data.len()
  }

  pub fn in_volume_len(&self) -> usize {
    self.inner_node.len()
  }

  pub fn len(&self) -> usize {
    self.out_of_volume_len() + self.in_volume_len()
  }

  pub fn maximum_depth(&self) -> usize {
    self.inner_node.maximum_depth()
  }

  pub fn center(&self) -> [f32; 3] {
    self.inner_node.center.clone()
  }

  pub fn half_size(&self) -> [f32; 3] {
    self.inner_node.half_size.clone()
  }

  pub fn find(&self, coord: &[f32; 3]) -> Option<&D> {
    if self.inner_node.coord_is_out_of_bounds(coord) {
      self
        .out_of_volume_data
        .iter()
        .position(|d| d.get_coord() == coord)
        .and_then(|idx| self.out_of_volume_data.get(idx))
    } else {
      self.inner_node.find(coord)
    }
  }

  pub fn remove(&mut self, coord: &[f32; 3]) -> Option<D> {
    let is_out_of_volume = self.inner_node.coord_is_out_of_bounds(coord);

    let removed_node = if is_out_of_volume {
      self
        .out_of_volume_data
        .iter()
        .position(|d| d.get_coord() == coord)
        .map(|idx| self.out_of_volume_data.remove(idx))
    } else {
      self.inner_node.remove(coord)
    };

    if removed_node.is_some() {
      if self.params.resize_tree_bounds && self.len() > self.params.tree_resize_minimum_population {
        self.try_to_shrink_whole_tree();
      }
    } else {
      warn!("Tried to remove {:?} but there was no such node", coord);
    }

    removed_node
  }

  fn try_to_shrink_whole_tree(&mut self) {
    if self.inner_node.len() == 0 {
      // Don't know how to grow if there are no out of tree nodes.
      return;
    }

    let mut in_tree_occupancy_ratio = 1f32 - (self.out_of_volume_len() as f32 / self.len() as f32);

    while in_tree_occupancy_ratio > 0.0
      && in_tree_occupancy_ratio > self.params.desired_tree_occupancy_ratio_max
    {
      if self.inner_node.is_leaf {
        // Can't shrink a leaf node
        break;
      }

      // UNWRAP: Guarded by prior check
      let mut child_populations = [0; 8];
      for (idx, ref child) in self.inner_node.children.iter().enumerate() {
        child_populations[idx] = child.as_ref().map(|c| c.len()).unwrap_or(0);
      }

      // UNWRAP: Fixed size
      let (max_idx, _) = child_populations
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .unwrap();

      let mut population_without_max_node = 0;
      for (idx, &pop) in child_populations.iter().enumerate() {
        if idx == max_idx {
          continue;
        }

        population_without_max_node += pop;
      }

      let potential_new_occupancy_ratio = 1f32
        - (((population_without_max_node + self.out_of_volume_len()) as f32) / self.len() as f32);

      if potential_new_occupancy_ratio < self.params.desired_tree_occupancy_ratio_min {
        // Not going to lower the occupancy below the threshold where we'd have to grow again
        break;
      }

      {
        let mut new_root_node = None;

        // UNWRAP: Guarded above by prior check
        mem::swap(&mut new_root_node, &mut self.inner_node.children[max_idx]);
        self
          .out_of_volume_data
          .append(&mut self.inner_node.extract_values());

        // Make our current root into the extracted sub-node
        // UNWRAP: Child node is known to exist (it had the max population)
        mem::swap(new_root_node.unwrap().deref_mut(), &mut self.inner_node);
      }

      in_tree_occupancy_ratio = potential_new_occupancy_ratio
    }
  }
}

impl<D: AsCoord, M: Default + Clone> OctreeRootNode<D, M> {
  pub fn in_place_map_reduce<MF, RF>(&mut self, mapper: &MF, reducer: &RF)
  where
    MF: Fn(&D) -> M,
    RF: Fn(M, M) -> M,
  {
    self.out_of_volume_metadata = self
      .out_of_volume_data
      .iter()
      .map(mapper)
      .fold(M::default(), reducer);
    self.inner_node.in_place_map_reduce(mapper, reducer);
  }
}

impl<D: AsCoord + Clone, M> OctreeRootNode<D, M> {
  pub fn map_reduce<MO: Clone, MF, RF>(
    &self,
    initial_value: MO,
    mapper: &MF,
    reducer: &RF,
  ) -> OctreeRootNode<D, MO>
  where
    MF: Fn(&D) -> MO,
    RF: Fn(MO, MO) -> MO,
  {
    let out_of_volume_metadata = self
      .out_of_volume_data
      .iter()
      .map(mapper)
      .fold(initial_value.clone(), reducer);
    let inner_node = self.inner_node.map_reduce(initial_value, mapper, reducer);
    OctreeRootNode {
      out_of_volume_data: self.out_of_volume_data.clone(),
      inner_node: inner_node,
      out_of_volume_metadata: out_of_volume_metadata,
      params: self.params.clone(),
    }
  }

  pub fn traverse_reduce<TO: Clone, TF, RF>(
    &self,
    initial_value: TO,
    traverser: &TF,
    reducer: &RF,
  ) -> TO
  where
    TF: Fn(NodeTraversalData<D, M>) -> ForwardTraversalResult<TO>,
    RF: Fn(TO, TO) -> TO,
  {
    let in_volume_result = self
      .inner_node
      .traverse_reduce(initial_value, traverser, reducer);

    if self.out_of_volume_data.is_empty() {
      return in_volume_result;
    }

    let traversal_data = NodeTraversalData {
      data: &self.out_of_volume_data,
      metadata: &self.out_of_volume_metadata,
      center: &self.inner_node.center,
      half_size: &self.inner_node.half_size,
      negate_specified_bouunds: true,
      // out_of_bounds is considered leaf
      is_leaf: true,
    };
    reducer(traverser(traversal_data).partial_result, in_volume_result)
  }
}

impl<D: AsCoord + UpdateCoord, M: Default> OctreeRootNode<D, M> {
  pub fn update(&mut self, from: &[f32; 3], to: [f32; 3]) {
    let is_out_of_volume = self.inner_node.coord_is_out_of_bounds(from);

    if is_out_of_volume {
      if let Some(idx) = self
        .out_of_volume_data
        .iter()
        .position(|d| d.get_coord() == from)
      {
        let new_is_out_of_volume = self.inner_node.coord_is_out_of_bounds(&to);

        if new_is_out_of_volume {
          // UNWRAP: Guarded by above check
          self
            .out_of_volume_data
            .get_mut(idx)
            .unwrap()
            .update_coord(to);
        } else {
          let mut data = self.out_of_volume_data.remove(idx);

          data.update_coord(to);
          self.inner_node.insert(data);
        }
      } else {
        warn!(
          "Tried to update {:?} but there was no such node (out of volume?)",
          from
        );
        return;
      }
    } else if let Some(mut data) = self.inner_node.remove(from) {
      data.update_coord(to);

      if self.inner_node.coord_is_out_of_bounds(&to) {
        self.out_of_volume_data.push(data)
      } else {
        self.inner_node.insert(data);
      }
    } else {
      warn!(
        "Tried to update {:?} but there was no such node (inside volume?)",
        from
      );
      return;
    }
    if self.params.resize_tree_bounds && self.len() > self.params.tree_resize_minimum_population {
      self.try_to_grow_whole_tree();
    }
  }
}

impl<D: AsCoord, M: Default> OctreeNode<D, M> {
  pub fn insert(&mut self, item: D) -> bool {
    debug_assert!(!self.coord_is_out_of_bounds(item.get_coord()));

    // Enqueue into own data and move on
    if self.is_leaf && self.data.len() < self.config.node_capacity {
      if self.coord_exists_in_self(item.get_coord()) {
        return false;
      }

      self.data.push(item);
      self.population += 1;
      return true;
    }

    // Become non-leaf, create 8 children, and populate them with own data, plus new entry and move
    // on
    if self.is_leaf && self.data.len() == self.config.node_capacity {
      if self.coord_exists_in_self(item.get_coord()) {
        return false;
      }

      self.is_leaf = false;

      // Take own data and distribute it among children
      {
        let mut data = Vec::new();
        mem::swap(&mut self.data, &mut data);

        for entry in data.drain(..) {
          let child_idx = self.find_child_index_for_coord(entry.get_coord());
          if self.children[child_idx].is_some() {
            // UNWRAP: Access guarded
            self.children[child_idx].as_mut().unwrap().insert(entry);
          } else {
            let NodeBounds { center, half_size } = self.determine_child_bounds(child_idx);
            self.children[child_idx] = Some(Box::new(OctreeNode {
              config: self.config.clone(),
              data: vec![entry],
              center: center,
              half_size: half_size,
              is_leaf: true,
              children: [None, None, None, None, None, None, None, None],
              population: 1,
              metadata: M::default(),
            }));
          }
        }
      }
    };

    // Add to appropriate child and move on
    let child_idx = self.find_child_index_for_coord(item.get_coord());
    let inserted = if self.children[child_idx].is_some() {
      // UNWRAP: Access guarded
      self.children[child_idx].as_mut().unwrap().insert(item)
    } else {
      let NodeBounds { center, half_size } = self.determine_child_bounds(child_idx);
      self.children[child_idx] = Some(Box::new(OctreeNode {
        config: self.config.clone(),
        data: vec![item],
        center: center,
        half_size: half_size,
        is_leaf: true,
        children: [None, None, None, None, None, None, None, None],
        population: 1,
        metadata: M::default(),
      }));
      true
    };

    if inserted {
      self.population += 1;
    }
    return inserted;
  }
}

impl<D: AsCoord, M> OctreeNode<D, M> {
  pub fn len(&self) -> usize {
    self.population
  }

  pub fn maximum_depth(&self) -> usize {
    // UNWRAP: `children` is of fixed size
    1usize
      + self
        .children
        .iter()
        .map(|c_opt| c_opt.as_ref().map(|c| c.maximum_depth()).unwrap_or(0usize))
        .max()
        .unwrap()
  }

  pub fn find(&self, coord: &[f32; 3]) -> Option<&D> {
    if self.coord_is_out_of_bounds(coord) {
      return None;
    }

    if self.is_leaf {
      if let Some(idx) = self.data.iter().position(|d| d.get_coord() == coord) {
        self.data.get(idx)
      } else {
        None
      }
    } else {
      let child_idx = self.find_child_index_for_coord(coord);
      return self.children[child_idx]
        .as_ref()
        .and_then(|c| c.find(coord));
    }
  }

  pub fn remove(&mut self, coord: &[f32; 3]) -> Option<D> {
    if let Some(RemovalDetails {
      data,
      should_consider_merging,
      ..
    }) = self.remove_internal(coord)
    {
      if should_consider_merging {
        // We need to merge (this probably means the entire tree needs to merge, as the root is
        // usually the node this is called on)
        self.merge()
      }
      Some(data)
    } else {
      None
    }
  }

  fn remove_internal(&mut self, coord: &[f32; 3]) -> Option<RemovalDetails<D>> {
    if self.coord_is_out_of_bounds(coord) {
      return None;
    }

    if self.is_leaf {
      if let Some(idx) = self.data.iter().position(|d| d.get_coord() == coord) {
        let data = self.data.remove(idx);
        let OctreeConfig {
          ref node_capacity,
          ref desired_node_occupancy_ratio_min,
          ..
        } = self.config;
        let should_consider_merging =
          ((self.data.len() as f32) / (*node_capacity as f32)) < *desired_node_occupancy_ratio_min;

        // N.B.: Retaining the number of values for an ongoing merge was considered, but it greatly
        // complicates the implementation without sufficient benefit. Consider that the highest
        // this value could be is desired_node_occupancy_ratio_min * node_capacity, since the value is
        // not retained if we're not going to merge the nodes. This means that avoiding recounting
        // these nodes would be saving very little work.
        self.population -= 1;
        return Some(RemovalDetails {
          data: data,
          should_consider_merging: should_consider_merging,
          merging_current_depth: 0,
        });
      } else {
        return None;
      }
    } else {
      let child_idx = self.find_child_index_for_coord(coord);

      let removal_from_child = self.children[child_idx]
        .as_mut()
        .and_then(|c| c.remove_internal(coord));
      if removal_from_child.is_none() {
        return None;
      }

      // At this point, we know for sure that a removal occurred
      self.population -= 1;

      // UNWRAP: Guaranteed to be present from prior guard
      let should_consider_merging = removal_from_child
        .as_ref()
        .map(|v| v.should_consider_merging)
        .unwrap();
      if !should_consider_merging {
        return removal_from_child;
      }

      // UNWRAP: Guaranteed to be present from prior guard
      let RemovalDetails {
        data,
        merging_current_depth,
        ..
      } = removal_from_child.unwrap();

      let OctreeConfig {
        ref node_capacity,
        ref desired_node_occupancy_ratio_min,
        ..
      } = self.config;
      let still_should_consider_merging =
        ((self.len() as f32) / (*node_capacity as f32)) < *desired_node_occupancy_ratio_min;
      if still_should_consider_merging {
        // Both our children, and ourselves, are capable to be merged. Propagate upward
        // (letting parent see if they should be merrged as well)
        return Some(RemovalDetails {
          data: data,
          should_consider_merging: true,
          merging_current_depth: merging_current_depth + 1,
        });
      } else if merging_current_depth > 0 {
        // There are valuable merges to be performed for our child, even if we can't be merged.
        // Perform them now.
        // UNWRAP: Guaranteed to be present from prior guard
        self.children[child_idx].as_mut().unwrap().merge();
      }

      // Either the merge was a dead end (child was leaf), or we already performed the merge
      return Some(RemovalDetails {
        data: data,
        should_consider_merging: false,
        merging_current_depth: 0, /* irrelevant, wont merge */
      });
    }
  }

  fn merge(&mut self) {
    debug_assert!(!self.len() <= self.config.node_capacity);

    self.data = self.extract_values();
    self.children = [None, None, None, None, None, None, None, None];
    self.is_leaf = true;
  }

  fn extract_values(&mut self) -> Vec<D> {
    // N.B.: This is not meant to be used as a general purpose draining operation
    // It will perform fairly poorly for large numbers of values due to recursive collect
    // operations.

    if self.is_leaf {
      if self.data.is_empty() {
        Vec::new()
      } else {
        let mut d = Vec::new();
        mem::swap(&mut d, &mut self.data);
        d
      }
    } else {
      // UNWRAP: Guaranteed to be present from self.is_leaf
      self
        .children
        .iter_mut()
        .flat_map(|c_opt| {
          c_opt
            .as_mut()
            .map(|c| c.extract_values())
            .unwrap_or(Vec::new())
            .into_iter()
        })
        .collect()
    }
  }

  fn determine_child_bounds(&self, idx: usize) -> NodeBounds {
    debug_assert!(idx < 8);

    let quarter_size = [
      self.half_size[X_INDEX] / 2.0,
      self.half_size[Y_INDEX] / 2.0,
      self.half_size[Z_INDEX] / 2.0,
    ];

    let x_dir = ((idx >> 2 & 1) * 2) as f32 - 1f32;
    let y_dir = ((idx >> 1 & 1) * 2) as f32 - 1f32;
    let z_dir = ((idx & 1) * 2) as f32 - 1f32;

    let x_center = self.center[X_INDEX] + (quarter_size[X_INDEX] * x_dir);
    let y_center = self.center[Y_INDEX] + (quarter_size[Y_INDEX] * y_dir);
    let z_center = self.center[Z_INDEX] + (quarter_size[Z_INDEX] * z_dir);

    NodeBounds {
      center: [x_center, y_center, z_center],
      half_size: quarter_size,
    }
  }

  fn coord_is_out_of_bounds(&self, coord: &[f32; 3]) -> bool {
    let max_x = self.center[X_INDEX] + self.half_size[X_INDEX];
    let min_x = self.center[X_INDEX] - self.half_size[X_INDEX];
    let max_y = self.center[Y_INDEX] + self.half_size[Y_INDEX];
    let min_y = self.center[Y_INDEX] - self.half_size[Y_INDEX];
    let max_z = self.center[Z_INDEX] + self.half_size[Z_INDEX];
    let min_z = self.center[Z_INDEX] - self.half_size[Z_INDEX];

    coord[X_INDEX] > max_x || coord[X_INDEX] < min_x || coord[Y_INDEX] > max_y
      || coord[Y_INDEX] < min_y || coord[Z_INDEX] > max_z || coord[Z_INDEX] < min_z
  }

  fn find_child_index_for_coord(&self, coord: &[f32; 3]) -> usize {
    // N.B. This is also useful for figuring out where a coordinate would be if the volume were
    // big enough

    let x_on_right = coord[X_INDEX] > self.center[X_INDEX];
    let y_on_right = coord[Y_INDEX] > self.center[Y_INDEX];
    let z_on_right = coord[Z_INDEX] > self.center[Z_INDEX];

    let x_idx_offset = if x_on_right { 4 } else { 0 };
    let y_idx_offset = if y_on_right { 2 } else { 0 };
    let z_idx_offset = if z_on_right { 1 } else { 0 };

    x_idx_offset + y_idx_offset + z_idx_offset
  }

  fn coord_exists_in_self(&self, coord: &[f32; 3]) -> bool {
    self.data.iter().any(|e| e.get_coord() == coord)
  }
}

impl<D: AsCoord, M: Default + Clone> OctreeNode<D, M> {
  pub fn in_place_map_reduce<MF, RF>(&mut self, mapper: &MF, reducer: &RF)
  where
    MF: Fn(&D) -> M,
    RF: Fn(M, M) -> M,
  {
    if self.is_leaf {
      self.metadata = self.data.iter().map(&mapper).fold(M::default(), &reducer);
    } else {
      let mut children_results = Vec::new();
      for child in self.children.iter_mut() {
        if let &mut Some(ref mut c) = child {
          c.in_place_map_reduce(mapper, reducer);
          children_results.push(c.metadata.clone());
        }
      }
      // UNWRAP: Guarded by filter
      self.metadata = children_results.into_iter().fold(M::default(), &reducer);
    }
  }
}

impl<D: AsCoord + Clone, M> OctreeNode<D, M> {
  pub fn map_reduce<MO: Clone, MF, RF>(
    &self,
    initial_value: MO,
    mapper: &MF,
    reducer: &RF,
  ) -> OctreeNode<D, MO>
  where
    MF: Fn(&D) -> MO,
    RF: Fn(MO, MO) -> MO,
  {
    if self.is_leaf {
      let metadata = self
        .data
        .iter()
        .map(&mapper)
        .fold(initial_value.clone(), &reducer);
      return OctreeNode {
        config: self.config.clone(),
        data: self.data.clone(),
        center: self.center.clone(),
        half_size: self.half_size.clone(),
        is_leaf: true,
        children: [None, None, None, None, None, None, None, None],
        population: self.population,
        metadata: metadata,
      };
    } else {
      let mut children = [None, None, None, None, None, None, None, None];
      for (idx, child) in self.children.iter().enumerate() {
        children[idx] = child
          .as_ref()
          .map(|c| Box::new(c.map_reduce(initial_value.clone(), mapper, reducer)));
      }
      // UNWRAP: Guarded by filter
      let metadata: MO = children
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.as_ref().unwrap().metadata.clone())
        .fold(initial_value, &reducer);

      return OctreeNode {
        config: self.config.clone(),
        data: self.data.clone(),
        center: self.center.clone(),
        half_size: self.half_size.clone(),
        is_leaf: false,
        children: children,
        population: self.population,
        metadata: metadata,
      };
    }
  }

  pub fn traverse_reduce<TO: Clone, TF, RF>(
    &self,
    initial_value: TO,
    traverser: &TF,
    reducer: &RF,
  ) -> TO
  where
    TF: Fn(NodeTraversalData<D, M>) -> ForwardTraversalResult<TO>,
    RF: Fn(TO, TO) -> TO,
  {
    let ForwardTraversalResult {
      should_continue,
      partial_result,
    } = {
      let traversal_data = NodeTraversalData {
        data: &self.data,
        metadata: &self.metadata,
        center: &self.center,
        half_size: &self.half_size,
        negate_specified_bouunds: false,
        is_leaf: self.is_leaf,
      };

      traverser(traversal_data)
    };

    if should_continue && !self.is_leaf {
      let mut children_results = Vec::new();
      for child in self.children.iter() {
        if let &Some(ref c) = child {
          children_results.push(c.traverse_reduce(initial_value.clone(), traverser, reducer));
        }
      }
      // UNWRAP: Guarded by filter
      return children_results
        .into_iter()
        .chain(iter::once(partial_result))
        .fold(initial_value, &reducer);
    } else {
      return partial_result;
    }
  }
}

impl<'a, D, M> NodeTraversalData<'a, D, M> {
  pub fn coord_is_out_of_bounds(&self, coord: &[f32; 3]) -> bool {
    let max_x = self.center[X_INDEX] + self.half_size[X_INDEX];
    let min_x = self.center[X_INDEX] - self.half_size[X_INDEX];
    let max_y = self.center[Y_INDEX] + self.half_size[Y_INDEX];
    let min_y = self.center[Y_INDEX] - self.half_size[Y_INDEX];
    let max_z = self.center[Z_INDEX] + self.half_size[Z_INDEX];
    let min_z = self.center[Z_INDEX] - self.half_size[Z_INDEX];

    coord[X_INDEX] > max_x || coord[X_INDEX] < min_x || coord[Y_INDEX] > max_y
      || coord[Y_INDEX] < min_y || coord[Z_INDEX] > max_z || coord[Z_INDEX] < min_z
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone, Debug)]
  struct PointMass {
    coord: [f32; 3],
    mass: f32,
  }

  impl AsCoord for PointMass {
    fn get_coord(&self) -> &[f32; 3] {
      &self.coord
    }
  }

  impl Default for PointMass {
    fn default() -> PointMass {
      PointMass {
        coord: [0f32, 0f32, 0f32],
        mass: 0f32,
      }
    }
  }

  #[test]
  fn test_octree_new() {
    let octree = OctreeRootNode::<[f32; 3]>::new(OctreeInitParams::default());
    assert_eq!(octree.len(), 0);
  }

  #[test]
  fn test_octree_insert_within_capacity() {
    // N.B: Out of volume elements don't count toward capacity
    // N.B: Duplicates don't count toward capacity
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [5f32, 5f32, 5f32];
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([-4.0, 0.0, 0.0]);
    octree.insert([4.0, 0.0, 0.0]);
    octree.insert([0.0, -4.0, 0.0]);
    octree.insert([0.0, 4.0, 0.0]);
    octree.insert([0.0, 0.0, -4.0]);
    octree.insert([0.0, 0.0, 4.0]);
    octree.insert([-4.0, -4.0, -4.0]);
    octree.insert([4.0, 4.0, 4.0]);

    // Duplicate
    octree.insert([4.0, 4.0, 4.0]);

    // Out of bounds
    octree.insert([0.0, 0.0, 5.1]);

    assert_eq!(octree.in_volume_len(), 8);
    assert_eq!(octree.out_of_volume_len(), 1);
    assert_eq!(octree.len(), 9);
    assert_eq!(octree.maximum_depth(), 1);
  }

  #[test]
  fn test_octree_insert_overcapacity() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [5f32, 5f32, 5f32];
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([-4.0, 0.0, 0.0]);
    octree.insert([4.0, 0.0, 0.0]);
    octree.insert([0.0, -4.0, 0.0]);
    octree.insert([0.0, 4.0, 0.0]);
    octree.insert([0.0, 0.0, -4.0]);
    octree.insert([0.0, 0.0, 4.0]);
    octree.insert([-4.0, -4.0, -4.0]);
    octree.insert([4.0, 4.0, 4.0]);
    octree.insert([2.0, 2.0, 2.0]);

    assert_eq!(octree.len(), 9);
    assert_eq!(octree.maximum_depth(), 2);
  }

  #[test]
  fn test_octree_insert_dense_splitting() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [100f32, 100f32, 100f32];
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([1.0, 1.0, 1.0]);
    octree.insert([2.0, 2.0, 2.0]);
    octree.insert([3.0, 3.0, 3.0]);
    octree.insert([4.0, 4.0, 4.0]);
    octree.insert([5.0, 5.0, 5.0]);
    octree.insert([6.0, 6.0, 6.0]);
    octree.insert([7.0, 7.0, 7.0]);
    octree.insert([8.0, 8.0, 8.0]);
    octree.insert([9.0, 9.0, 9.0]);

    assert_eq!(octree.len(), 9);
    // (100 -> 50 -> 25 -> 12.5 -> 6.25) plus one for the root
    assert_eq!(octree.maximum_depth(), 6);
  }

  #[test]
  fn test_octree_insert_duplicates_dont_end_the_world() {
    // N.B.: Without proper duplicate handling, node overcapacity with duplicates would cause an
    // infinite cycle of insertions into ever smaller nodes.
    //
    let mut octree = OctreeRootNode::<[f32; 3]>::new(OctreeInitParams::default());
    for _ in 0..10 {
      octree.insert([0.0, 0.0, 0.0]);
    }

    // If we got this far, we're ok!
  }

  #[test]
  fn test_octree_remove_can_perform_mega_merge() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [100f32, 100f32, 100f32];
    // Merge any trees at or below 5 nodes: (a bit plus 5 / 8)
    octree_init_params.desired_node_occupancy_ratio_min = 0.626;
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([1.0, 1.0, 1.0]);
    octree.insert([2.0, 2.0, 2.0]);
    octree.insert([3.0, 3.0, 3.0]);
    octree.insert([4.0, 4.0, 4.0]);
    octree.insert([5.0, 5.0, 5.0]);
    octree.insert([6.0, 6.0, 6.0]);
    octree.insert([7.0, 7.0, 7.0]);
    octree.insert([8.0, 8.0, 8.0]);
    octree.insert([9.0, 9.0, 9.0]);

    assert_eq!(octree.len(), 9);
    // (200 -> 100 -> 50 -> 25 -> 12.5 -> 6.25)
    assert_eq!(octree.maximum_depth(), 6);

    // Due to the tight grouping of the points, the first removals won't be very impactful
    // N.B: The minimum node will *want* to merge, but the parent will be above the minimum
    // occupancy, so no merge will happen.
    {
      let value = octree.remove(&[9.0, 9.0, 9.0]);
      assert_eq!(value, Some([9.0, 9.0, 9.0]));
      assert_eq!(octree.len(), 8);
      assert_eq!(octree.maximum_depth(), 6);
    }
    {
      let value = octree.remove(&[8.0, 8.0, 8.0]);
      assert_eq!(value, Some([8.0, 8.0, 8.0]));
      assert_eq!(octree.len(), 7);
      assert_eq!(octree.maximum_depth(), 6);
    }
    {
      let value = octree.remove(&[7.0, 7.0, 7.0]);
      assert_eq!(value, Some([7.0, 7.0, 7.0]));
      assert_eq!(octree.len(), 6);
      assert_eq!(octree.maximum_depth(), 6);
    }
    // Once we hit 5 nodes remaining, a major merge should reduce us down to the root
    {
      let value = octree.remove(&[6.0, 6.0, 6.0]);
      assert_eq!(value, Some([6.0, 6.0, 6.0]));
      assert_eq!(octree.len(), 5);
      assert_eq!(octree.maximum_depth(), 1);
    }
  }

  #[test]
  fn test_remove_can_remove_out_of_bound_things() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [100f32, 100f32, 100f32];
    // Merge any trees at or below 5 nodes: (a bit plus 5 / 8)
    octree_init_params.desired_node_occupancy_ratio_min = 0.626;
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([-100.1, 100.1, 100.1]);

    let data = octree.remove(&[-100.1, 100.1, 100.1]);
    assert_eq!(data, Some([-100.1, 100.1, 100.1]));
    assert_eq!(octree.len(), 0);
  }

  #[test]
  fn test_octree_remove_can_merge_incrementally() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [100f32, 100f32, 100f32];
    // Merge any trees at or below 5 nodes: (a bit plus 5 / 8)
    octree_init_params.desired_node_occupancy_ratio_min = 0.626;
    octree_init_params.resize_tree_bounds = false;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([-1.0, 1.0, 1.0]);
    assert_eq!(octree.len(), 1);
    octree.insert([1.0, 1.0, 1.0]);
    assert_eq!(octree.len(), 2);
    octree.insert([2.0, 2.0, 2.0]);
    assert_eq!(octree.len(), 3);
    octree.insert([3.0, 3.0, 3.0]);
    assert_eq!(octree.len(), 4);
    octree.insert([4.0, 4.0, 4.0]);
    assert_eq!(octree.len(), 5);
    octree.insert([5.0, 5.0, 5.0]);
    assert_eq!(octree.len(), 6);
    octree.insert([6.0, 6.0, 6.0]);
    assert_eq!(octree.len(), 7);
    octree.insert([7.0, 7.0, 7.0]);
    assert_eq!(octree.len(), 8);
    octree.insert([8.0, 8.0, 8.0]);
    assert_eq!(octree.len(), 9);
    octree.insert([9.0, 9.0, 9.0]);

    assert_eq!(octree.len(), 10);
    // (200 -> 100 -> 50 -> 25 -> 12.5 -> 6.25)
    assert_eq!(octree.maximum_depth(), 6);

    // Due to the tight grouping of the points, the first removals won't be very impactful
    // N.B: The minimum node will *want* to merge, but the parent will be above the minimum
    // occupancy, so no merge will happen.
    {
      let value = octree.remove(&[9.0, 9.0, 9.0]);
      assert_eq!(value, Some([9.0, 9.0, 9.0]));
      assert_eq!(octree.len(), 9);
      assert_eq!(octree.maximum_depth(), 6);
    }
    {
      let value = octree.remove(&[8.0, 8.0, 8.0]);
      assert_eq!(value, Some([8.0, 8.0, 8.0]));
      assert_eq!(octree.len(), 8);
      assert_eq!(octree.maximum_depth(), 6);
    }
    {
      let value = octree.remove(&[7.0, 7.0, 7.0]);
      assert_eq!(value, Some([7.0, 7.0, 7.0]));
      assert_eq!(octree.len(), 7);
      assert_eq!(octree.maximum_depth(), 6);
    }
    // Once we hit 6 nodes remaining, a subtree merge should reduce us to two levels
    {
      let value = octree.remove(&[6.0, 6.0, 6.0]);
      assert_eq!(value, Some([6.0, 6.0, 6.0]));
      assert_eq!(octree.len(), 6);
      assert_eq!(octree.maximum_depth(), 2);
    }
    // Once we hit 5 nodes remaining, a full merge should reduce us to the root
    {
      let value = octree.remove(&[5.0, 5.0, 5.0]);
      assert_eq!(value, Some([5.0, 5.0, 5.0]));
      assert_eq!(octree.len(), 5);
      assert_eq!(octree.maximum_depth(), 1);
    }
  }

  #[test]
  fn test_find_works_as_expected() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.node_capacity = 2;
    octree_init_params.desired_tree_occupancy_ratio_min = 0.999;
    octree_init_params.desired_tree_occupancy_ratio_max = 0.999;
    octree_init_params.resize_tree_bounds = true;
    octree_init_params.tree_resize_minimum_population = 0;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    octree.insert([-1.0, 1.0, 2.0]);
    octree.insert([1.0, 1.0, 1.0]);
    octree.insert([2.0, 2.0, 2.0]);
    octree.insert([3.0, 3.0, 3.0]);

    // All nodes should be locatabable
    assert!(octree.find(&[-1.0, 1.0, 2.0]).is_some());
    assert!(octree.find(&[1.0, 1.0, 1.0]).is_some());
    assert!(octree.find(&[2.0, 2.0, 2.0]).is_some());
    assert!(octree.find(&[3.0, 3.0, 3.0]).is_some());
  }

  #[test]
  fn test_find_works_after_resize() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.node_capacity = 2;
    octree_init_params.desired_tree_occupancy_ratio_min = 0.999;
    octree_init_params.desired_tree_occupancy_ratio_max = 0.999;
    octree_init_params.resize_tree_bounds = true;
    octree_init_params.tree_resize_minimum_population = 0;
    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    // Should work after resize as well
    {
      println!("octree: {:#?}", octree);
      octree.insert([100.1, 1.0, 1.0]);
      println!("octree: {:#?}", octree);
      assert!(octree.find(&[100.1, 1.0, 1.0]).is_some());
    }

    {
      println!("octree: {:#?}", octree);
      octree.insert([-600.1, 1.0, 1.0]);
      println!("octree: {:#?}", octree);

      {
        let mut octree_init_params = OctreeInitParams::default();
        octree_init_params.center = [-1000.0, 600.0, 600.0];
        octree_init_params.half_size = [800.0, 800.0, 800.0];
        octree_init_params.node_capacity = 2;
        octree_init_params.desired_tree_occupancy_ratio_min = 0.999;
        octree_init_params.desired_tree_occupancy_ratio_max = 0.999;
        octree_init_params.resize_tree_bounds = true;
        octree_init_params.tree_resize_minimum_population = 1000;
        let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
        octree.insert([100.1, 1.0, 1.0]);
        octree.insert([-600.1, 1.0, 1.0]);
        println!("octree manual: {:#?}", octree);
      }

      assert!(octree.find(&[100.1, 1.0, 1.0]).is_some());
      assert!(octree.find(&[-600.1, 1.0, 1.0]).is_some());
    }
  }

  #[test]
  fn test_octree_can_resize_root() {
    let mut octree_init_params = OctreeInitParams::default();
    octree_init_params.half_size = [100f32, 100f32, 100f32];
    // Disable grouping sparse nodes (can cause sparse octree to ossify into a giant node without
    // resizing)
    octree_init_params.desired_node_occupancy_ratio_min = 0.000;
    // Force the tree to enclose all points in the volume
    octree_init_params.desired_tree_occupancy_ratio_min = 0.999;
    octree_init_params.desired_tree_occupancy_ratio_max = 0.999;
    octree_init_params.resize_tree_bounds = true;
    octree_init_params.tree_resize_minimum_population = 0;

    let mut octree = OctreeRootNode::<[f32; 3]>::new(octree_init_params);
    //println!("octree: {:#?}", octree);
    octree.insert([1.0, 1.0, 1.0]);

    // A single insertion should cause the tree to resize
    {
      octree.insert([101.0, 101.0, 101.0]);
      println!("octree: {:#?}", octree);
      assert_eq!(octree.len(), 2);
      let half_size = octree.half_size();
      println!("center {:?}", octree.center());
      println!("half_size {:?}", half_size);
      assert!(half_size[X_INDEX] > 199f32 && half_size[X_INDEX] < 201f32);
      assert!(half_size[Y_INDEX] > 199f32 && half_size[Y_INDEX] < 201f32);
      assert!(half_size[Z_INDEX] > 199f32 && half_size[Z_INDEX] < 201f32);
      let center = octree.center();
      assert!(center[X_INDEX] > 99f32 && center[X_INDEX] < 101f32);
      assert!(center[Y_INDEX] > 99f32 && center[Y_INDEX] < 101f32);
      assert!(center[Z_INDEX] > 99f32 && center[Z_INDEX] < 101f32);
      assert!(octree.find(&[1.0, 1.0, 1.0]).is_some());
      assert!(octree.find(&[101.0, 101.0, 101.0]).is_some());
    }
  }

  #[test]
  fn test_mapreduce() {
    let octree_init_params = OctreeInitParams::default();
    let mut octree = OctreeRootNode::<PointMass>::new(octree_init_params);

    octree.insert(PointMass {
      coord: [1.0, 2.0, 3.0],
      mass: 5000.0,
    });
    octree.insert(PointMass {
      coord: [8.0, 6.0, 30.0],
      mass: 200.0,
    });
    octree.insert(PointMass {
      coord: [1.0, 6.0, 3.0],
      mass: 50.0,
    });
    octree.insert(PointMass {
      coord: [10.0, -200.0, 5.0],
      mass: 500.0,
    });
    octree.insert(PointMass {
      coord: [1000.0, -2.0, 5.0],
      mass: 800.0,
    });
    octree.insert(PointMass {
      coord: [-5.0, -200.0, 500.0],
      mass: 800.0,
    });

    let bh_tree = octree.map_reduce(
      PointMass {
        coord: [0.0, 0.0, 0.0],
        mass: 0.0,
      },
      &map_point_mass,
      &reduce_point_masses,
    );

    println!("{:?}", bh_tree.out_of_volume_metadata);
    assert!(
      bh_tree.out_of_volume_metadata.mass > 2099.0 && bh_tree.out_of_volume_metadata.mass < 2101.0
    );

    println!("{:?}", bh_tree.inner_node.metadata);
    assert!(bh_tree.inner_node.metadata.mass > 5249.0 && bh_tree.inner_node.metadata.mass < 5251.0);
  }

  #[test]
  fn test_inplace_mapreduce() {
    let octree_init_params = OctreeInitParams::default();
    let mut octree = OctreeRootNode::<PointMass, PointMass>::new(octree_init_params);

    octree.insert(PointMass {
      coord: [1.0, 2.0, 3.0],
      mass: 5000.0,
    });
    octree.insert(PointMass {
      coord: [8.0, 6.0, 30.0],
      mass: 200.0,
    });
    octree.insert(PointMass {
      coord: [1.0, 6.0, 3.0],
      mass: 50.0,
    });
    octree.insert(PointMass {
      coord: [10.0, -200.0, 5.0],
      mass: 500.0,
    });
    octree.insert(PointMass {
      coord: [1000.0, -2.0, 5.0],
      mass: 800.0,
    });
    octree.insert(PointMass {
      coord: [-5.0, -200.0, 500.0],
      mass: 800.0,
    });

    octree.in_place_map_reduce(&map_point_mass, &reduce_point_masses);

    println!("{:?}", octree.out_of_volume_metadata);
    assert!(
      octree.out_of_volume_metadata.mass > 2099.0 && octree.out_of_volume_metadata.mass < 2101.0
    );

    println!("{:?}", octree.inner_node.metadata);
    assert!(octree.inner_node.metadata.mass > 5249.0 && octree.inner_node.metadata.mass < 5251.0);
  }

  fn map_point_mass(i: &PointMass) -> PointMass {
    i.clone()
  }

  fn reduce_point_masses(i: PointMass, j: PointMass) -> PointMass {
    if i.mass == 0.0 {
      return j;
    } else if j.mass == 0.0 {
      return i;
    }

    let mass_sum = i.mass + j.mass;
    let i_mass_fraction = i.mass / mass_sum;
    let j_mass_fraction = j.mass / mass_sum;

    // Positive + Negative mass that exactly cancels? WTF?
    debug_assert!(mass_sum != 0.0);

    return PointMass {
      coord: [
        i.coord[X_INDEX] * i_mass_fraction + j.coord[X_INDEX] * j_mass_fraction,
        i.coord[Y_INDEX] * i_mass_fraction + j.coord[Y_INDEX] * j_mass_fraction,
        i.coord[Z_INDEX] * i_mass_fraction + j.coord[Z_INDEX] * j_mass_fraction,
      ],
      mass: mass_sum,
    };
  }
}
