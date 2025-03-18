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

  TransparentNorth,
  TransparentSouth,
  TransparentEast,
  TransparentWest,
  TransparentTop,
  TransparentBottom,

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

      BlockMeshLocation::TransparentNorth => {
        BlockMeshLocation::TransparentSouth
      }
      BlockMeshLocation::TransparentSouth => {
        BlockMeshLocation::TransparentNorth
      }
      BlockMeshLocation::TransparentEast => BlockMeshLocation::TransparentWest,
      BlockMeshLocation::TransparentWest => BlockMeshLocation::TransparentEast,
      BlockMeshLocation::TransparentTop => BlockMeshLocation::TransparentBottom,
      BlockMeshLocation::TransparentBottom => BlockMeshLocation::TransparentTop,

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

  transparent_north_indices: Vec<u16>,
  transparent_south_indices: Vec<u16>,
  transparent_east_indices: Vec<u16>,
  transparent_west_indices: Vec<u16>,
  transparent_top_indices: Vec<u16>,
  transparent_bottom_indices: Vec<u16>,

  inside_indices: Vec<u16>,

  draw_category: usize,
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

      BlockMeshLocation::TransparentNorth => &self.transparent_north_indices,
      BlockMeshLocation::TransparentSouth => &self.transparent_south_indices,
      BlockMeshLocation::TransparentEast => &self.transparent_east_indices,
      BlockMeshLocation::TransparentWest => &self.transparent_west_indices,
      BlockMeshLocation::TransparentTop => &self.transparent_top_indices,
      BlockMeshLocation::TransparentBottom => &self.transparent_bottom_indices,

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

      BlockMeshLocation::TransparentNorth => &self.transparent_north_indices,
      BlockMeshLocation::TransparentSouth => &self.transparent_south_indices,
      BlockMeshLocation::TransparentEast => &self.transparent_east_indices,
      BlockMeshLocation::TransparentWest => &self.transparent_west_indices,
      BlockMeshLocation::TransparentTop => &self.transparent_top_indices,
      BlockMeshLocation::TransparentBottom => &self.transparent_bottom_indices,

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

  pub fn draw_category(&self) -> usize {
    self.draw_category
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
      north_indices: vec![16, 18, 19, 16, 19, 17],
      south_indices: vec![22, 20, 21, 22, 21, 23],
      east_indices: vec![0, 1, 2, 0, 2, 3],
      west_indices: vec![5, 4, 7, 5, 7, 6],
      top_indices: vec![9, 8, 10, 9, 10, 11],
      bottom_indices: vec![14, 15, 13, 14, 13, 12],

      transparent_north_indices: vec![],
      transparent_south_indices: vec![],
      transparent_east_indices: vec![],
      transparent_west_indices: vec![],
      transparent_top_indices: vec![],
      transparent_bottom_indices: vec![],

      inside_indices: vec![],

      draw_category: 0,
    }
  }

  pub fn model_simple_transparent() -> Self {
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
      north_indices: vec![],
      south_indices: vec![],
      east_indices: vec![],
      west_indices: vec![],
      top_indices: vec![],
      bottom_indices: vec![],

      transparent_north_indices: vec![16, 18, 19, 16, 19, 17],
      transparent_south_indices: vec![22, 20, 21, 22, 21, 23],
      transparent_east_indices: vec![0, 1, 2, 0, 2, 3],
      transparent_west_indices: vec![5, 4, 7, 5, 7, 6],
      transparent_top_indices: vec![9, 8, 10, 9, 10, 11],
      transparent_bottom_indices: vec![14, 15, 13, 14, 13, 12],

      inside_indices: vec![],

      draw_category: 1,
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
      north_indices: vec![16, 18, 19, 16, 19, 17],
      south_indices: vec![22, 20, 21, 22, 21, 23],
      east_indices: vec![0, 1, 2, 0, 2, 3],
      west_indices: vec![5, 4, 7, 5, 7, 6],
      top_indices: vec![9, 8, 10, 9, 10, 11],
      bottom_indices: vec![14, 15, 13, 14, 13, 12],

      transparent_north_indices: vec![],
      transparent_south_indices: vec![],
      transparent_east_indices: vec![],
      transparent_west_indices: vec![],
      transparent_top_indices: vec![],
      transparent_bottom_indices: vec![],

      inside_indices: vec![],

      draw_category: 0,
    }
  }
}
