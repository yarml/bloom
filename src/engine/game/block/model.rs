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
  pub fn new(
    vertices: &[Vertex],
    north_indices: &[u16],
    south_indices: &[u16],
    east_indices: &[u16],
    west_indices: &[u16],
    top_indices: &[u16],
    bottom_indices: &[u16],
    inside_indices: &[u16],
  ) -> Self {
    Self {
      vertices: vertices.into(),
      north_indices: north_indices.into(),
      south_indices: south_indices.into(),
      east_indices: east_indices.into(),
      west_indices: west_indices.into(),
      top_indices: top_indices.into(),
      bottom_indices: bottom_indices.into(),
      inside_indices: inside_indices.into(),
    }
  }

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
    match location {
      BlockMeshLocation::North => &self.north_indices,
      BlockMeshLocation::South => &self.south_indices,
      BlockMeshLocation::East => &self.east_indices,
      BlockMeshLocation::West => &self.west_indices,
      BlockMeshLocation::Top => &self.top_indices,
      BlockMeshLocation::Bottom => &self.bottom_indices,
      BlockMeshLocation::Inside => &self.inside_indices,
    }
    .len()
      != 0
  }

  pub fn vertices_at(&self, origin: Point3<f32>) -> Vec<Vertex> {
    self
      .vertices
      .iter()
      .map(|vertex| {
        let mut cvertex = vertex.clone();
        cvertex.translate(origin.to_vec());
        cvertex
      })
      .collect()
  }
}
