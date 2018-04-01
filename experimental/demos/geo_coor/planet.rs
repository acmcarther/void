extern crate chrono;
extern crate geometry;
extern crate icosphere;
extern crate log;
extern crate rand;

struct Planet {
  plates: Vec<Plate>,
}

struct Plate {}

// A coordinate on an icosphere-derived surface
// A base icosphere has 20 faces:
// https://en.wikipedia.org/wiki/Icosphere
//
// An arbitrary location (down to 17 iterations of precision) can be specified by successive
// subdivision of an icosphere.
//
// The process of icosphere subdivision iteratively splits each face four ways -- meaning that a
// subface can be identified with two bits (i.e. (00, 01, 10, 11)). We use two 16 bit numbers to
// identify the sucessive levels of specificity
struct Coordinate {
  // Which fundamental face to start traversal from
  // TODO(acmcarther): Define iteration order
  // VALID RANGE: 0 - 19
  root_face: u8,
  // Count of bits to consider for face travesal
  resolution: u8,
  // A series of coordinate subdivision "bits" for a face.
  // ... Each successive bit is another level of resolution.
  face_traversal_left: u16,
  // A series of coordinate subdivision "bits" for a face.
  // ... Each successive bit is another level of resolution.
  face_traversal_right: u16,
}

#[cfg(test)]
mod tests {
  use super::*;
  use icosphere;
  use geometry::Mesh;
  use geometry::Vertex;

  #[test]
  fn face_count() {
    println!(
      "Icosphere 0 face count: {}",
      icosphere::icosphere(0).num_faces()
    );
    println!(
      "Icosphere 1 face count: {}",
      icosphere::icosphere(1).num_faces()
    );
    println!(
      "Icosphere 2 face count: {}",
      icosphere::icosphere(2).num_faces()
    );
    println!(
      "Icosphere 3 face count: {}",
      icosphere::icosphere(3).num_faces()
    );

    assert_eq!(1, 2);
  }
}
