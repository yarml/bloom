use std::rc::Rc;

use crate::engine::{
  camera::Camera,
  game::block::{
    instance::{BlockInstance, BlockPosition},
    Block,
  },
};

pub const CHUNK_DIMEN: usize = 32;
const CHUNK_BLOCK_COUNT: usize = CHUNK_DIMEN * CHUNK_DIMEN * CHUNK_DIMEN;

// A chunk is a 32x32x32 array of block instances
pub struct Chunk {
  origin: BlockPosition,
  blocks: [Option<BlockInstance>; CHUNK_BLOCK_COUNT],
}

impl Chunk {
  pub fn new(origin: BlockPosition) -> Self {
    Self {
      origin,
      blocks: [const { None }; CHUNK_BLOCK_COUNT],
    }
  }

  pub fn origin(&self) -> BlockPosition {
    self.origin
  }

  pub fn place_block(&mut self, relpos: BlockPosition, block: Rc<Block>) {
    self.blocks[relpos.x as usize
      + relpos.y as usize * CHUNK_DIMEN
      + relpos.z as usize * CHUNK_DIMEN * CHUNK_DIMEN] =
      Some(BlockInstance::new(block, self.origin + relpos));
  }

  pub fn is_visible(&self, camera: &Camera) -> bool {
    true
  }
}
