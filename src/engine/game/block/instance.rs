use std::{
  ops::{Add, Rem, Sub},
  rc::Rc,
};

use cgmath::Vector3;

use super::Block;

#[derive(Clone, Copy, PartialEq)]
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
}

impl Into<Vector3<f32>> for BlockPosition {
  fn into(self) -> Vector3<f32> {
    Vector3 {
      x: self.x as f32,
      y: self.y as f32,
      z: self.z as f32,
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
