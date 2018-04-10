extern crate chrono;
extern crate log;
extern crate node;

use std::mem;
use std::iter::Iterator;
use std::slice::Iter;
use std::slice::IterMut;
use node::EntityId;
use node::NodeService;
use node::RunError;
use node::TickContext;
use node::NodeServiceMetadata;

pub const OCCUPANCY_RATIO_UNDEFINED: f32 = -1.0;

/** A densely populated contiguous entity store */
struct DenseStore<T> {
  data: Vec<Option<T>>,
  metadata: DenseStoreMetadata,
}

struct DenseStoreMetadata {
  pub size_of_t: usize,
  pub len: usize,
  pub collection_capacity: usize,
  pub occupancy_ratio: f32,
}

struct DenseStoreIter<'a, T: 'a> {
  dense_store_iter: Iter<'a, Option<T>>,
  len: usize,
  capacity: usize,
  index: usize,
  real_elements_iterated_count: usize,
}

struct DenseStoreIterMut<'a, T: 'a> {
  dense_store_iter_mut: IterMut<'a, Option<T>>,
  len: usize,
  capacity: usize,
  index: usize,
  real_elements_iterated_count: usize,
}

impl<T> DenseStore<T> {
  pub fn new() -> DenseStore<T> {
    DenseStore {
      data: Vec::new(),
      metadata: DenseStoreMetadata {
        size_of_t: mem::size_of::<T>(),
        len: 0,
        collection_capacity: 0,
        occupancy_ratio: OCCUPANCY_RATIO_UNDEFINED,
      },
    }
  }

  pub fn insert(&mut self, entity_id: EntityId, data: T) -> Option<T> {
    let entity_id_usize = entity_id as usize;
    let mut did_update_capacity = false;
    while entity_id_usize >= self.data.capacity() {
      // Double the current capacity
      let capacity = self.data.capacity() + 1;
      self.data.reserve(capacity);
      did_update_capacity = true;
    }

    if did_update_capacity {
      self.metadata.collection_capacity = self.data.capacity()
    }

    let required_new_elements = self.data.capacity() - self.data.len();

    for _ in 0..required_new_elements {
      self.data.push(None);
    }

    let mut data_opt = Some(data);
    {
      // Guaranteed to be present -- data resized above to at least contain entity_id
      mem::swap(self.data.get_mut(entity_id_usize).unwrap(), &mut data_opt);
    }

    if data_opt.is_none() {
      self.metadata.len = self.metadata.len + 1;
    }

    if data_opt.is_none() || did_update_capacity {
      self.metadata.occupancy_ratio =
        (self.metadata.len as f32) / (self.metadata.collection_capacity as f32);
    }

    data_opt
  }

  pub fn remove(&mut self, entity_id: EntityId) -> Option<T> {
    let entity_id_usize = entity_id as usize;
    if entity_id_usize > self.data.len() {
      return None;
    }

    let mut data = None;
    {
      // UNWRAP: Known to exist from above guard
      mem::swap(self.data.get_mut(entity_id_usize).unwrap(), &mut data);
    }

    if data.is_some() {
      self.metadata.len = self.metadata.len - 1;
    }

    data
  }

  pub fn get_entity(&self, entity_id: EntityId) -> Option<&T> {
    self.data.get(entity_id as usize).and_then(|s| s.as_ref())
  }

  pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<&mut T> {
    self
      .data
      .get_mut(entity_id as usize)
      .and_then(|s| s.as_mut())
  }

  fn get_internal(&self, index: usize) -> Option<&T> {
    self.data.get(index).and_then(|s| s.as_ref())
  }

  fn get_internal_mut(&mut self, index: usize) -> Option<&mut T> {
    self.data.get_mut(index).and_then(|s| s.as_mut())
  }

  pub fn capacity(&self) -> usize {
    self.metadata.collection_capacity
  }

  pub fn len(&self) -> usize {
    self.metadata.len
  }

  pub fn iter(&self) -> DenseStoreIter<T> {
    DenseStoreIter {
      dense_store_iter: self.data.iter(),
      len: self.len(),
      capacity: self.capacity(),
      index: 0,
      real_elements_iterated_count: 0,
    }
  }

  pub fn iter_mut(&mut self) -> DenseStoreIterMut<T> {
    DenseStoreIterMut {
      len: self.len(),
      capacity: self.capacity(),
      dense_store_iter_mut: self.data.iter_mut(),
      index: 0,
      real_elements_iterated_count: 0,
    }
  }
}

impl<'a, T> Iterator for DenseStoreIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<&'a T> {
    if self.real_elements_iterated_count == self.len {
      return None;
    }

    // Skipping check for index > capacity, case implicitly handled above

    for index in self.index..self.capacity {
      let item = self.dense_store_iter.next();
      // Inner iterator is done.
      if item.is_none() {
        self.index = self.capacity;
        return None;
      }

      let item_inner = item.unwrap();
      if item_inner.is_some() {
        self.index = index + 1;
        self.real_elements_iterated_count = self.real_elements_iterated_count + 1;
        return item_inner.as_ref();
      }
    }

    self.index = self.capacity;
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.len - self.real_elements_iterated_count;
    (remaining, Some(remaining))
  }
}

impl<'a, T> Iterator for DenseStoreIterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<&'a mut T> {
    if self.real_elements_iterated_count == self.len {
      return None;
    }

    // Skipping check for index > capacity, case implicitly handled above

    for index in self.index..self.capacity {
      let item = self.dense_store_iter_mut.next();
      // Inner iterator is done.
      if item.is_none() {
        self.index = self.capacity;
        return None;
      }

      let item_inner = item.unwrap();
      if item_inner.is_some() {
        self.index = index + 1;
        self.real_elements_iterated_count = self.real_elements_iterated_count + 1;
        return item_inner.as_mut();
      }
    }

    self.index = self.capacity;
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.len - self.real_elements_iterated_count;
    (remaining, Some(remaining))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dense_store_basic_operations_work() {
    let mut ds = DenseStore::new();
    assert_eq!(ds.len(), 0);

    ds.insert(0, 5u32);
    assert_eq!(ds.len(), 1);

    ds.insert(0, 10u32);
    assert_eq!(ds.len(), 1);

    ds.insert(1, 15u32);
    assert_eq!(ds.len(), 2);

    ds.remove(5000);
    assert_eq!(ds.len(), 2);

    let res = ds.remove(1);
    assert_eq!(res, Some(15));
    assert_eq!(ds.len(), 1);

    ds.insert(1000, 20);
    assert_eq!(ds.len(), 2);

    assert_eq!(ds.get_entity(1000), Some(&20));
    assert_eq!(ds.get_entity(1), None);
    assert_eq!(ds.get_entity(0), Some(&10));
  }

  #[test]
  fn dense_store_iters_work() {
    let mut ds = DenseStore::new();
    ds.insert(0, 100);
    ds.insert(10, 1000);
    ds.insert(100, 10000);

    let sum: u32 = ds.iter().map(|v| v.clone()).sum();
    assert_eq!(sum, 11100);

    for x in ds.iter_mut() {
      *x = *x * 2;
    }

    let sum: u32 = ds.iter().map(|v| v.clone()).sum();
    assert_eq!(sum, 22200);
  }
}
