use cgmath::{EuclideanSpace, Point3};

use crate::engine::model::Vertex;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum BlockMeshLocation {
  North,
  South,
  East,
  West,
  Top,
  Bottom,
  Inside,
}

impl BlockMeshLocation {
  pub fn opposite(&self) -> BlockMeshLocation {
    match self {
      BlockMeshLocation::North => BlockMeshLocation::South,
      BlockMeshLocation::South => BlockMeshLocation::North,
      BlockMeshLocation::East => BlockMeshLocation::West,
      BlockMeshLocation::West => BlockMeshLocation::East,
      BlockMeshLocation::Top => BlockMeshLocation::Bottom,
      BlockMeshLocation::Bottom => BlockMeshLocation::Top,
      BlockMeshLocation::Inside => BlockMeshLocation::Inside,
    }
  }
}

pub struct BlockModel {
  vertices: Vec<Vertex>,

  north_indices: Vec<u16>,
  south_indices: Vec<u16>,
  east_indices: Vec<u16>,
  west_indices: Vec<u16>,
  top_indices: Vec<u16>,
  bottom_indices: Vec<u16>,
  inside_indices: Vec<u16>,
}

impl BlockModel {
  pub fn indices_of(
    &self,
    location: BlockMeshLocation,
    shift: u16,
  ) -> Vec<u16> {
    match location {
      BlockMeshLocation::North => &self.north_indices,
      BlockMeshLocation::South => &self.south_indices,
      BlockMeshLocation::East => &self.east_indices,
      BlockMeshLocation::West => &self.west_indices,
      BlockMeshLocation::Top => &self.top_indices,
      BlockMeshLocation::Bottom => &self.bottom_indices,
      BlockMeshLocation::Inside => &self.inside_indices,
    }
    .iter()
    .map(|index| index + shift)
    .collect()
  }

  pub fn has_face_at(&self, location: BlockMeshLocation) -> bool {
    !match location {
      BlockMeshLocation::North => &self.north_indices,
      BlockMeshLocation::South => &self.south_indices,
      BlockMeshLocation::East => &self.east_indices,
      BlockMeshLocation::West => &self.west_indices,
      BlockMeshLocation::Top => &self.top_indices,
      BlockMeshLocation::Bottom => &self.bottom_indices,
      BlockMeshLocation::Inside => &self.inside_indices,
    }
    .is_empty()
  }

  pub fn vertices_at(&self, origin: Point3<f32>) -> Vec<Vertex> {
    self
      .vertices
      .iter()
      .map(|vertex| {
        let mut cvertex = *vertex;
        cvertex.translate(origin.to_vec());
        cvertex
      })
      .collect()
  }

  pub fn model_simple() -> Self {
    Self {
      vertices: vec![
        // East
        ((0.0, 0.0, 1.0), (0.0, 1.0)).into(), // 0
        ((1.0, 0.0, 1.0), (1.0, 1.0)).into(), // 1
        ((1.0, 1.0, 1.0), (1.0, 0.0)).into(), // 2
        ((0.0, 1.0, 1.0), (0.0, 0.0)).into(), // 3
        // West
        ((0.0, 0.0, 0.0), (1.0, 1.0)).into(), // 4
        ((1.0, 0.0, 0.0), (0.0, 1.0)).into(), // 5
        ((1.0, 1.0, 0.0), (0.0, 0.0)).into(), // 6
        ((0.0, 1.0, 0.0), (1.0, 0.0)).into(), // 7
        // Top
        ((1.0, 1.0, 1.0), (1.0, 1.0)).into(), // 8 -> 2
        ((0.0, 1.0, 1.0), (0.0, 1.0)).into(), // 9 -> 3
        ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 10 -> 6
        ((0.0, 1.0, 0.0), (0.0, 0.0)).into(), // 11 -> 7
        // Bottom
        ((0.0, 0.0, 1.0), (0.0, 0.0)).into(), // 12 -> 0
        ((1.0, 0.0, 1.0), (1.0, 0.0)).into(), // 13 -> 1
        ((0.0, 0.0, 0.0), (0.0, 1.0)).into(), // 14 -> 4
        ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 15 -> 5
        // North
        ((1.0, 0.0, 1.0), (0.0, 1.0)).into(), // 16 -> 1
        ((1.0, 1.0, 1.0), (0.0, 0.0)).into(), // 17 -> 2
        ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 18 -> 5
        ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 19 -> 6
        // South
        ((0.0, 0.0, 1.0), (1.0, 1.0)).into(), // 20 -> 0
        ((0.0, 1.0, 1.0), (1.0, 0.0)).into(), // 21 -> 3
        ((0.0, 0.0, 0.0), (0.0, 1.0)).into(), // 22 -> 4
        ((0.0, 1.0, 0.0), (0.0, 0.0)).into(), // 23 -> 7
      ],
      north_indices: vec![16, 18, 19, 16, 19, 17], // North
      south_indices: vec![22, 20, 21, 22, 21, 23], // South
      east_indices: vec![0, 1, 2, 0, 2, 3],        // East
      west_indices: vec![5, 4, 7, 5, 7, 6],        // West
      top_indices: vec![9, 8, 10, 9, 10, 11],      // Top
      bottom_indices: vec![14, 15, 13, 14, 13, 12], // Bottom
      inside_indices: vec![],                      // Inside
    }
  }

  pub fn model_side_vert() -> Self {
    Self {
      vertices: vec![
        // East
        ((0.0, 0.0, 1.0), (0.0, 1.0)).into(), // 0
        ((1.0, 0.0, 1.0), (0.5, 1.0)).into(), // 1
        ((1.0, 1.0, 1.0), (0.5, 0.0)).into(), // 2
        ((0.0, 1.0, 1.0), (0.0, 0.0)).into(), // 3
        // West
        ((0.0, 0.0, 0.0), (0.5, 1.0)).into(), // 4
        ((1.0, 0.0, 0.0), (0.0, 1.0)).into(), // 5
        ((1.0, 1.0, 0.0), (0.0, 0.0)).into(), // 6
        ((0.0, 1.0, 0.0), (0.5, 0.0)).into(), // 7
        // Top
        ((1.0, 1.0, 1.0), (1.0, 1.0)).into(), // 8 -> 2
        ((0.0, 1.0, 1.0), (0.5, 1.0)).into(), // 9 -> 3
        ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 10 -> 6
        ((0.0, 1.0, 0.0), (0.5, 0.0)).into(), // 11 -> 7
        // Bottom
        ((0.0, 0.0, 1.0), (0.5, 0.0)).into(), // 12 -> 0
        ((1.0, 0.0, 1.0), (1.0, 0.0)).into(), // 13 -> 1
        ((0.0, 0.0, 0.0), (0.5, 1.0)).into(), // 14 -> 4
        ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 15 -> 5
        // North
        ((1.0, 0.0, 1.0), (0.0, 1.0)).into(), // 16 -> 1
        ((1.0, 1.0, 1.0), (0.0, 0.0)).into(), // 17 -> 2
        ((1.0, 0.0, 0.0), (0.5, 1.0)).into(), // 18 -> 5
        ((1.0, 1.0, 0.0), (0.5, 0.0)).into(), // 19 -> 6
        // South
        ((0.0, 0.0, 1.0), (0.5, 1.0)).into(), // 20 -> 0
        ((0.0, 1.0, 1.0), (0.5, 0.0)).into(), // 21 -> 3
        ((0.0, 0.0, 0.0), (0.0, 1.0)).into(), // 22 -> 4
        ((0.0, 1.0, 0.0), (0.0, 0.0)).into(), // 23 -> 7
      ],
      north_indices: vec![16, 18, 19, 16, 19, 17], // North
      south_indices: vec![22, 20, 21, 22, 21, 23], // South
      east_indices: vec![0, 1, 2, 0, 2, 3],        // East
      west_indices: vec![5, 4, 7, 5, 7, 6],        // West
      top_indices: vec![9, 8, 10, 9, 10, 11],      // Top
      bottom_indices: vec![14, 15, 13, 14, 13, 12], // Bottom
      inside_indices: vec![],                      // Inside
    }
  }
}
