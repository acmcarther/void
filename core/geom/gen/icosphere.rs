extern crate geometry;
extern crate itertools;

use geometry::Mesh;
use geometry::Vertex;
use itertools::Itertools;
use std::collections::HashMap;

pub struct IcosphereMeshBuilder {
  pub vertices: Vec<(f32, f32, f32)>,
  pub faces: Vec<(u16, u16, u16)>,
}

impl IcosphereMeshBuilder {
  pub fn to_mesh(self) -> Mesh {
    Mesh {
      // TODO(acmcarther): These uv coords are nonsensical
      vertices: self
        .vertices
        .into_iter()
        .map(|(x, y, z)| Vertex {
          pos: [x, y, z],
          norm: [x, y, z],
        })
        .collect(),
      // TODO(acmcarther): Not terribly efficient, but easy to implement
      indices: self
        .faces
        .into_iter()
        .flat_map(|(i1, i2, i3)| vec![i1, i2, i3])
        .collect(),
    }
  }
}

fn normalize(point: (f32, f32, f32)) -> (f32, f32, f32) {
  let (x, y, z) = point;
  let len = ((x * x + y * y + z * z) as f32).sqrt();
  (x / len, y / len, z / len)
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
      normalize((-1.0, radius_part, 0.0)),
      normalize((1.0, radius_part, 0.0)),
      normalize((-1.0, -radius_part, 0.0)),
      normalize((1.0, -radius_part, 0.0)),
      normalize((0.0, -1.0, radius_part)),
      normalize((0.0, 1.0, radius_part)),
      normalize((0.0, -1.0, -radius_part)),
      normalize((0.0, 1.0, -radius_part)),
      normalize((radius_part, 0.0, -1.0)),
      normalize((radius_part, 0.0, 1.0)),
      normalize((-radius_part, 0.0, -1.0)),
      normalize((-radius_part, 0.0, 1.0)),
    ],
    faces: vec![
      (0, 11, 5),
      (0, 5, 1),
      (0, 1, 7),
      (0, 7, 10),
      (0, 10, 11),
      (1, 5, 9),
      (5, 11, 4),
      (11, 10, 2),
      (10, 7, 6),
      (7, 1, 8),
      (3, 9, 4),
      (3, 4, 2),
      (3, 2, 6),
      (3, 6, 8),
      (3, 8, 9),
      (4, 9, 5),
      (2, 4, 11),
      (6, 2, 10),
      (8, 6, 7),
      (9, 8, 1),
    ],
  };

  let mut mid_point_cache = HashMap::new();

  (0..iterations).foreach(|_| {
    icosphere_mesh_builder.faces = icosphere_mesh_builder
      .faces
      .clone()
      .into_iter()
      .flat_map(|(i1, i2, i3)| {
        let mids: Vec<u16> = vec![(i1, i2), (i2, i3), (i3, i1)]
          .into_iter()
          .map(|(first, second)| {
            let (si1, si2) = if first < second {
              (first, second)
            } else {
              (second, first)
            };
            mid_point_cache
              .entry((si1, si2))
              .or_insert_with(|| {
                let &(v1x, v1y, v1z) = icosphere_mesh_builder.vertices.get(si1 as usize).unwrap();
                let &(v2x, v2y, v2z) = icosphere_mesh_builder.vertices.get(si2 as usize).unwrap();
                let result = normalize((v1x + v2x / 2.0, v1y + v2y / 2.0, v1z + v2z / 2.0));
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

        vec![(i1, m1, m3), (i2, m2, m1), (i3, m3, m2), (m1, m2, m3)]
      })
      .collect()
  });

  icosphere_mesh_builder.to_mesh()
}

#[cfg(test)]
mod tests {
  use super::*;
  use icosphere;
  use geometry::Mesh;
  use geometry::Vertex;

  #[test]
  fn face_count() {
    assert_eq!(icosphere(0).num_faces(), 20);
    assert_eq!(icosphere(1).num_faces(), 80);
    assert_eq!(icosphere(2).num_faces(), 320);
    assert_eq!(icosphere(3).num_faces(), 1280);
  }
}
