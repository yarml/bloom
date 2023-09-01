use std::rc::Rc;

use self::chunk::Chunk;

use super::block::{instance::BlockPosition, Block};

pub mod chunk;

pub struct World {
  loaded_chunks: Vec<Chunk>,
}

impl World {
  pub fn new() -> Self {
    Self {
      loaded_chunks: vec![],
    }
  }

  pub fn place_block(
    &mut self,
    position: BlockPosition,
    block: Rc<Block>,
  ) -> Result<(), ()> {
    let chunk_origin = position % 32;
    let relpos = position - chunk_origin;
    match self
      .loaded_chunks
      .iter_mut()
      .find(|chunk| chunk.origin() == chunk_origin)
    {
      Some(chunk) => {
        chunk.place_block(relpos, block);
        Ok(())
      }
      None => Err(()),
    }
  }
}
