#[macro_use]
extern crate log;

use std::mem;
use std::ops::DerefMut;
use std::ptr;

type Index = u64;

pub trait WithCoord {
  fn get_coord(&self) -> &[f32; 3];
}

impl WithCoord for [f32; 3] {
  fn get_coord(&self) -> &[f32; 3] {
    &self
  }
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
pub struct OctreeRootNode<D> {
  params: OctreeInitParams,
  out_of_volume_data: Vec<D>,
  inner_node: OctreeNode<D>,
}

#[derive(Debug)]
struct OctreeNode<D> {
  config: OctreeConfig,
  data: Vec<D>,
  center: [f32; 3],
  half_size: [f32; 3],
  is_leaf: bool,
  population: usize,
  children: Option<[Box<OctreeNode<D>>; 8]>,
}

struct RemovalDetails<D> {
  data: D,
  should_consider_merging: bool,
  merging_current_depth: usize,
}

const X_INDEX: usize = 0;
const Y_INDEX: usize = 1;
const Z_INDEX: usize = 2;

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

impl<D: WithCoord> OctreeRootNode<D> {
  pub fn new(params: OctreeInitParams) -> OctreeRootNode<D> {
    OctreeRootNode {
      out_of_volume_data: Vec::new(),
      inner_node: OctreeNode {
        config: OctreeConfig {
          node_capacity: params.node_capacity.clone(),
          desired_node_occupancy_ratio_min: params.desired_node_occupancy_ratio_min.clone(),
        },
        data: Vec::new(),
        center: params.center.clone(),
        half_size: params.half_size.clone(),
        is_leaf: true,
        children: None,
        population: 0,
      },
      params: params,
    }
  }

  pub fn insert(&mut self, item: D) {
    let is_out_of_volume = self.inner_node.coord_is_out_of_bounds(item.get_coord());

    if is_out_of_volume {
      let already_exists_out_of_volume = {
        let coord = item.get_coord();
        self
          .out_of_volume_data
          .iter()
          .any(|e| e.get_coord() == coord)
      };

      if !already_exists_out_of_volume {
        self.out_of_volume_data.push(item);
        if self.params.resize_tree_bounds && self.len() > self.params.tree_resize_minimum_population
        {
          self.try_to_grow_whole_tree();
        }
      }
    } else {
      self.inner_node.insert(item);
    }
  }

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

  pub fn remove(&mut self, coord: &[f32; 3]) -> Option<D> {
    let removed_node = self.inner_node.remove(coord);

    if removed_node.is_some() {
      if self.params.resize_tree_bounds && self.len() > self.params.tree_resize_minimum_population {
        self.try_to_shrink_whole_tree();
      }
    }

    removed_node
  }

  fn try_to_grow_whole_tree(&mut self) {
    if self.out_of_volume_data.is_empty() {
      // Don't know how to grow if there are no out of tree nodes.
      return;
    }

    let mut in_tree_occupancy_ratio =
      1f32 - (self.out_of_volume_len() as f32 / self.inner_node.len() as f32);

    while in_tree_occupancy_ratio < self.params.desired_tree_occupancy_ratio_min {
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
        let x_offset = -(parent_direction_relative_to_child[X_INDEX] * double_size[X_INDEX]);
        let y_offset = -(parent_direction_relative_to_child[Y_INDEX] * double_size[Y_INDEX]);
        let z_offset = -(parent_direction_relative_to_child[Z_INDEX] * double_size[Z_INDEX]);
        // Parent center is current center *minus* offset relative to child
        let x_center = self.inner_node.center[X_INDEX] - x_offset;
        let y_center = self.inner_node.center[Y_INDEX] - y_offset;
        let z_center = self.inner_node.center[Z_INDEX] - z_offset;
        [x_center, y_center, z_center]
      };

      // Find the exact opposite index (this node from the new parent's perspective
      // Bitflip of each index component does the trick
      let mut current_root_child_idx = max_idx;
      current_root_child_idx ^= 0;
      current_root_child_idx ^= 1;
      current_root_child_idx ^= 2;

      unsafe {
        // Take ownership of inner node without initializing a replacement
        let mut inner_node = std::mem::uninitialized();
        mem::swap(&mut inner_node, &mut self.inner_node);

        let mut new_outer_node = OctreeNode {
          config: inner_node.config.clone(),
          data: Vec::new(),
          center: new_center.clone(),
          // This looks confusing, but see `OctreeNode::insert` to understand how this is similar to
          // (but opposite from) insertion
          half_size: double_size.clone(),
          is_leaf: true,
          children: None,
          population: 0,
        };

        new_outer_node.population = inner_node.population;
        new_outer_node.is_leaf = false;
        new_outer_node.children = Some({
          let mut new_nodes: [Box<OctreeNode<D>>; 8] = mem::uninitialized();

          for x_split in 0..2 {
            let x_offset = inner_node.half_size[X_INDEX] * (-1f32 + (2f32 * x_split as f32));
            let x_center = new_center[X_INDEX] + x_offset;
            for y_split in 0..2 {
              let y_offset = inner_node.half_size[Y_INDEX] * (-1f32 + (2f32 * y_split as f32));
              let y_center = new_center[Y_INDEX] + y_offset;
              for z_split in 0..2 {
                let z_offset = inner_node.half_size[Z_INDEX] * (-1f32 + (2f32 * z_split as f32));
                let z_center = new_center[Z_INDEX] + z_offset;
                let node = Box::new(OctreeNode {
                  config: new_outer_node.config.clone(),
                  data: Vec::new(),
                  center: [x_center, y_center, z_center],
                  half_size: inner_node.half_size.clone(),
                  is_leaf: true,
                  children: None,
                  population: 0,
                });

                let node_index = (x_split * 4) + (y_split * 2) + z_split;

                ptr::write(&mut new_nodes[node_index], node);
              }
            }
          }
          new_nodes
        });


        // Replace the empty node (where our former inner_node belongs)
        // ... but dont `forget` it, because it is a structurally complete node and needs to be
        // dropped.
        let mut boxed_inner_node = Box::new(inner_node);
        mem::swap(
          // UNWRAP: Known to be populated from above code
          &mut new_outer_node.children.as_mut().unwrap()[current_root_child_idx],
          &mut boxed_inner_node,
        );

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
      in_tree_occupancy_ratio =
        1f32 - (self.out_of_volume_len() as f32 / self.inner_node.len() as f32);
    }
  }

  fn try_to_shrink_whole_tree(&mut self) {
    if self.inner_node.len() == 0 {
      // Don't know how to grow if there are no out of tree nodes.
      return;
    }

    println!("sanity check");

    let mut in_tree_occupancy_ratio =
      1f32 - (self.out_of_volume_len() as f32 / self.inner_node.len() as f32);

    while in_tree_occupancy_ratio > self.params.desired_tree_occupancy_ratio_max {
      if self.inner_node.children.is_none() {
        // Can't shrink the root node
        break;
      }

      // UNWRAP: Guarded by prior check
      let mut child_populations = [0; 8];
      for (idx, child) in self
        .inner_node
        .children
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
      {
        child_populations[idx] = child.len();
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

      println!("pop w/o .. {}", population_without_max_node);

      let potential_new_occupancy_ratio = 1f32
        - (((population_without_max_node + self.out_of_volume_len()) as f32)
          / self.inner_node.len() as f32);

      if potential_new_occupancy_ratio < self.params.desired_tree_occupancy_ratio_min {
        // Not going to lower the occupancy below the threshold where we'd have to grow again
        break;
      }

      {
        let mut new_root_node = Box::new(OctreeNode {
          config: self.inner_node.config.clone(),
          data: Vec::new(),
          center: [-123.0, 456.0, -789.0],
          half_size: [1.0, -1.0, 1.0],
          is_leaf: true,
          children: None,
          population: 0,
        });

        // UNWRAP: Guarded above by prior check
        mem::swap(
          &mut new_root_node,
          &mut self.inner_node.children.as_mut().unwrap()[max_idx],
        );
        self
          .out_of_volume_data
          .append(&mut self.inner_node.extract_values());

        // Make our current root into the extracted sub-node
        mem::swap(new_root_node.deref_mut(), &mut self.inner_node);
      }

      in_tree_occupancy_ratio = potential_new_occupancy_ratio
    }
  }
}

impl<D: WithCoord> OctreeNode<D> {
  pub fn insert(&mut self, item: D) -> bool {
    debug_assert!(!self.coord_is_out_of_bounds(item.get_coord()));

    // Enqueue into own data and move on
    if self.is_leaf && self.data.len() < self.config.node_capacity {
      if self.coord_exists_in_self(item.get_coord()) {
        println!(
          "Tried to insert into octree with already existing coord: {:?}",
          item.get_coord()
        );
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
        println!(
          "Tried to insert into octree with already existing coord: {:?}",
          item.get_coord()
        );
        return false;
      }

      self.is_leaf = false;
      self.children = Some(unsafe {
        let mut new_nodes: [Box<OctreeNode<D>>; 8] = mem::uninitialized();

        let quarter_size = [
          self.half_size[X_INDEX] / 2.0,
          self.half_size[Y_INDEX] / 2.0,
          self.half_size[Z_INDEX] / 2.0,
        ];

        // N.B: Center heuristic
        //
        // To find the next center, we need to take the last center, and either add one quarter node
        // size, or subtract one quarter node size. We can do a trick with the "index" of the split
        // (0 or 1, per axis) to branchlessly perform that operation.
        for x_split in 0..2 {
          let x_offset = quarter_size[X_INDEX] * (-1f32 + (2f32 * x_split as f32));
          let x_center = self.center[X_INDEX] + x_offset;
          for y_split in 0..2 {
            let y_offset = quarter_size[Y_INDEX] * (-1f32 + (2f32 * y_split as f32));
            let y_center = self.center[Y_INDEX] + y_offset;
            for z_split in 0..2 {
              let z_offset = quarter_size[Z_INDEX] * (-1f32 + (2f32 * z_split as f32));
              let z_center = self.center[Z_INDEX] + z_offset;
              let node = Box::new(OctreeNode {
                config: self.config.clone(),
                data: Vec::new(),
                center: [x_center, y_center, z_center],
                half_size: quarter_size.clone(),
                is_leaf: true,
                children: None,
                population: 0,
              });

              let node_index = (x_split * 4) + (y_split * 2) + z_split;

              ptr::write(&mut new_nodes[node_index], node);
            }
          }
        }
        new_nodes
      });

      // Take own data and distribute it among children
      {
        let mut data = Vec::new();
        mem::swap(&mut self.data, &mut data);

        for entry in data.drain(..) {
          let child_idx = self.find_child_index_for_coord(entry.get_coord());
          // UNWRAP: `children` created earlier in function
          (self.children.as_mut().unwrap()[child_idx]).insert(entry);
        }
      }
    };

    // Add to appropriate child and move on
    let child_idx = self.find_child_index_for_coord(item.get_coord());
    // UNWRAP: non_leaf octree must have children
    let inserted = (self.children.as_mut().unwrap()[child_idx]).insert(item);
    if inserted {
      self.population += 1;
    }
    return inserted;
  }

  pub fn len(&self) -> usize {
    self.population
  }

  pub fn maximum_depth(&self) -> usize {
    if let Some(ref children) = self.children {
      // UNWRAP: Always an exact number of children
      1usize + children.iter().map(|c| c.maximum_depth()).max().unwrap()
    } else {
      1usize
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

      // UNWRAP: `children` guaranteed by !is_leaf
      let removal_from_child = (self.children.as_mut().unwrap()[child_idx]).remove_internal(coord);
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
        println!("DEBUG: performing submerge");
        // There are valuable merges to be performed for our child, even if we can't be merged.
        // Perform them now.
        // UNWRAP: Guaranteed to be present from prior guard
        (self.children.as_mut().unwrap()[child_idx]).merge();
      }
      println!("Merge happened or died because it was too shallow");

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
    self.children = None;
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
        .as_mut()
        .unwrap()
        .iter_mut()
        .flat_map(|c| c.extract_values().into_iter())
        .collect()
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

#[cfg(test)]
mod tests {
  use super::*;

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
    println!("octree: {:#?}", octree);

    // A single insertion should cause the tree to resize
    {
      octree.insert([101.0, 101.0, 101.0]);
      println!("octree: {:#?}", octree);
      assert_eq!(octree.len(), 1);
      let half_size = octree.half_size();
      assert!(half_size[X_INDEX] > 199f32 && half_size[X_INDEX] < 201f32);
      assert!(half_size[Y_INDEX] > 199f32 && half_size[Y_INDEX] < 201f32);
      assert!(half_size[Z_INDEX] > 199f32 && half_size[Z_INDEX] < 201f32);
      let center = octree.center();
      assert!(center[X_INDEX] > 199f32 && center[X_INDEX] < 201f32);
      assert!(center[Y_INDEX] > 199f32 && center[Y_INDEX] < 201f32);
      assert!(center[Z_INDEX] > 199f32 && center[Z_INDEX] < 201f32);
    }

    // And now a super far away point
    {
      octree.insert([-6000.0, 101.0, 10001.0]);
      println!("octree: {:#?}", octree);
      let half_size = octree.half_size();
      assert!(half_size[X_INDEX] > 6399f32 && half_size[X_INDEX] < 6401f32);
      assert!(half_size[Y_INDEX] > 6399f32 && half_size[Y_INDEX] < 6401f32);
      assert!(half_size[Z_INDEX] > 6399f32 && half_size[Z_INDEX] < 6401f32);
    }

    // And then remove that point
    // Should be back down to a sane size (approx the initial size, with volume recentered)
    {
      let value = octree.remove(&[-6000.0, 101.0, 10001.0]);
      println!("octree: {:#?}", octree);
      assert_eq!(value, Some([-6000.0, 101.0, 10001.0]));
      let half_size = octree.half_size();
      assert!(half_size[X_INDEX] > 99f32 && half_size[X_INDEX] < 101f32);
      assert!(half_size[Y_INDEX] > 99f32 && half_size[Y_INDEX] < 101f32);
      assert!(half_size[Z_INDEX] > 99f32 && half_size[Z_INDEX] < 101f32);
      let center = octree.center();
      assert!(center[X_INDEX] > 99f32 && center[X_INDEX] < 101f32);
      assert!(center[Y_INDEX] > 99f32 && center[Y_INDEX] < 101f32);
      assert!(center[Z_INDEX] > 99f32 && center[Z_INDEX] < 101f32);
    }
  }
}
