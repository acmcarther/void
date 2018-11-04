extern crate math;
extern crate gfx_basics;
extern crate itertools;

use math::vector::Vector3;
use gfx_basics::Mesh;
use gfx_basics::Vertex;
use itertools::Itertools;
use std::collections::HashMap;

pub struct IcosphereMeshBuilder {
  pub vertices: Vec<Vector3<f32>>,
  pub faces: Vec<Vector3<u16>>,
}

impl IcosphereMeshBuilder {
  pub fn to_mesh(self) -> Mesh {
    Mesh {
      // TODO(acmcarther): These uv coords are nonsensical
      vertices: self
        .vertices
        .into_iter()
        .map(|vector| Vertex {
          pos: vector.clone(),
          norm: vector,
        })
        .collect(),
      // TODO(acmcarther): Not terribly efficient, but easy to implement
      indices: self
        .faces
        .into_iter()
        .flat_map(|vector| vec![*vector.x(), *vector.y(), *vector.z()])
        .collect(),
    }
  }
}

/**
 * Creates a sphere iteratively
 *
 * Adapted from http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
 */
pub fn icosphere(iterations: u32) -> Mesh {
  let radius_part = (1.0 + (5.0 as f32).sqrt()) / 2.0;
  let mut icosphere_mesh_builder = IcosphereMeshBuilder {
    vertices: vec![
      Vector3::new(-1.0, radius_part, 0.0).normalized(),
      Vector3::new(1.0, radius_part, 0.0).normalized(),
      Vector3::new(-1.0, -radius_part, 0.0).normalized(),
      Vector3::new(1.0, -radius_part, 0.0).normalized(),
      Vector3::new(0.0, -1.0, radius_part).normalized(),
      Vector3::new(0.0, 1.0, radius_part).normalized(),
      Vector3::new(0.0, -1.0, -radius_part).normalized(),
      Vector3::new(0.0, 1.0, -radius_part).normalized(),
      Vector3::new(radius_part, 0.0, -1.0).normalized(),
      Vector3::new(radius_part, 0.0, 1.0).normalized(),
      Vector3::new(-radius_part, 0.0, -1.0).normalized(),
      Vector3::new(-radius_part, 0.0, 1.0).normalized(),
    ],
    faces: vec![
      Vector3::new(0, 11, 5),
      Vector3::new(0, 5, 1),
      Vector3::new(0, 1, 7),
      Vector3::new(0, 7, 10),
      Vector3::new(0, 10, 11),
      Vector3::new(1, 5, 9),
      Vector3::new(5, 11, 4),
      Vector3::new(11, 10, 2),
      Vector3::new(10, 7, 6),
      Vector3::new(7, 1, 8),
      Vector3::new(3, 9, 4),
      Vector3::new(3, 4, 2),
      Vector3::new(3, 2, 6),
      Vector3::new(3, 6, 8),
      Vector3::new(3, 8, 9),
      Vector3::new(4, 9, 5),
      Vector3::new(2, 4, 11),
      Vector3::new(6, 2, 10),
      Vector3::new(8, 6, 7),
      Vector3::new(9, 8, 1),
    ],
  };

  let mut mid_point_cache = HashMap::new();

  (0..iterations).foreach(|_| {
    icosphere_mesh_builder.faces = icosphere_mesh_builder
      .faces
      .clone()
      .into_iter()
      .flat_map(|face| {
        let mids: Vec<u16> = vec![face.xy(), face.yz(), face.zx()]
          .into_iter()
          .map(|line| {

            let (si1, si2) = if line.x() < line.y() {
              (*line.x(), *line.y())
            } else {
              (*line.y(), *line.x())
            };
            mid_point_cache
              .entry((si1, si2))
              .or_insert_with(|| {
                let result = {
                  let v1 = icosphere_mesh_builder.vertices.get(si1 as usize).unwrap();
                  let v2 = icosphere_mesh_builder.vertices.get(si2 as usize).unwrap();
                  Vector3::new(v1.x() + v2.x() / 2.0, v1.y() + v2.y() / 2.0, v1.z() + v2.z() / 2.0).normalized()
                };
                icosphere_mesh_builder.vertices.push(result);
                let idx = icosphere_mesh_builder.vertices.len() - 1;
                idx as u16
              })
              .clone()
          })
          .collect();

        let (m1, m2, m3): (u16, u16, u16) = (
          mids.get(0).unwrap().clone(),
          mids.get(1).unwrap().clone(),
          mids.get(2).unwrap().clone(),
        );

        vec![
          Vector3::new(*face.x(), m1, m3),
          Vector3::new(*face.y(), m2, m1),
          Vector3::new(*face.z(), m3, m2),
          Vector3::new(m1, m2, m3),
        ]
      })
      .collect()
  });

  icosphere_mesh_builder.to_mesh()
}

#[cfg(test)]
mod tests {
  use super::*;
  use icosphere;
  use gfx_basics::Mesh;
  use gfx_basics::Vertex;

  #[test]
  fn face_count() {
    assert_eq!(icosphere(0).num_faces(), 20);
    assert_eq!(icosphere(1).num_faces(), 80);
    assert_eq!(icosphere(2).num_faces(), 320);
    assert_eq!(icosphere(3).num_faces(), 1280);
  }
}
