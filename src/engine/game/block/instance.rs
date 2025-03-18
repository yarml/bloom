use std::{
  fmt::Display,
  ops::{Add, Div, Rem, Sub},
  rc::Rc,
};

use cgmath::{Point3, Vector3};

use crate::engine::game::world::chunk::CHUNK_DIMEN;

use super::{model::BlockMeshLocation, Block};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

#[derive(Clone)]
pub struct BlockInstance {
  block: Rc<Block>,
  position: BlockPosition,
}

impl BlockInstance {
  pub fn new(block: Rc<Block>, position: BlockPosition) -> Self {
    Self { block, position }
  }

  pub fn block_type(&self) -> &Block {
    &self.block
  }

  pub fn position(&self) -> BlockPosition {
    self.position
  }
}

impl BlockPosition {
  pub fn north(&self) -> Self {
    Self {
      x: self.x + 1,
      y: self.y,
      z: self.z,
    }
  }
  pub fn south(&self) -> Self {
    Self {
      x: self.x - 1,
      y: self.y,
      z: self.z,
    }
  }
  pub fn east(&self) -> Self {
    Self {
      x: self.x,
      y: self.y,
      z: self.z + 1,
    }
  }
  pub fn west(&self) -> Self {
    Self {
      x: self.x,
      y: self.y,
      z: self.z - 1,
    }
  }
  pub fn top(&self) -> Self {
    Self {
      x: self.x,
      y: self.y + 1,
      z: self.z,
    }
  }
  pub fn bottom(&self) -> Self {
    Self {
      x: self.x,
      y: self.y - 1,
      z: self.z,
    }
  }

  pub fn is_valid_chunk_relpos(&self) -> bool {
    let chunk_range = 0..CHUNK_DIMEN as i32;
    chunk_range.contains(&self.x)
      && chunk_range.contains(&self.y)
      && chunk_range.contains(&self.z)
  }

  pub fn neighbour(&self, location: BlockMeshLocation) -> Self {
    match location {
      BlockMeshLocation::North => self.north(),
      BlockMeshLocation::South => self.south(),
      BlockMeshLocation::East => self.east(),
      BlockMeshLocation::West => self.west(),
      BlockMeshLocation::Top => self.top(),
      BlockMeshLocation::Bottom => self.bottom(),

      BlockMeshLocation::TransparentNorth => self.north(),
      BlockMeshLocation::TransparentSouth => self.south(),
      BlockMeshLocation::TransparentEast => self.east(),
      BlockMeshLocation::TransparentWest => self.west(),
      BlockMeshLocation::TransparentTop => self.top(),
      BlockMeshLocation::TransparentBottom => self.bottom(),

      BlockMeshLocation::Inside => *self,
    }
  }
}

impl Display for BlockPosition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "x={},y={},z={}", self.x, self.y, self.z)
  }
}

impl From<BlockPosition> for Vector3<f32> {
  fn from(val: BlockPosition) -> Self {
    Self {
      x: val.x as f32,
      y: val.y as f32,
      z: val.z as f32,
    }
  }
}

impl From<BlockPosition> for Point3<f32> {
  fn from(val: BlockPosition) -> Self {
    Self {
      x: val.x as f32,
      y: val.y as f32,
      z: val.z as f32,
    }
  }
}

impl From<(i32, i32, i32)> for BlockPosition {
  fn from(value: (i32, i32, i32)) -> Self {
    Self {
      x: value.0,
      y: value.1,
      z: value.2,
    }
  }
}

impl Add for BlockPosition {
  type Output = BlockPosition;

  fn add(self, rhs: Self) -> Self::Output {
    BlockPosition {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub for BlockPosition {
  type Output = BlockPosition;

  fn sub(self, rhs: Self) -> Self::Output {
    BlockPosition {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Rem<i32> for BlockPosition {
  type Output = BlockPosition;

  fn rem(self, rhs: i32) -> Self::Output {
    BlockPosition {
      x: self.x % rhs,
      y: self.y % rhs,
      z: self.z % rhs,
    }
  }
}

impl Div<i32> for BlockPosition {
  type Output = BlockPosition;

  fn div(self, rhs: i32) -> Self::Output {
    BlockPosition {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

impl Display for BlockInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "position=({}), block_type={:?}",
      self.position,
      self.block.name()
    )
  }
}
