use std::{collections::HashMap, rc::Rc};

use wgpu::Device;

use crate::engine::{camera::Camera, mesh::Mesh};

use self::chunk::Chunk;

use super::block::{instance::BlockPosition, registry::BlockRegistry, Block};

pub mod chunk;

pub struct World {
  loaded_chunks: HashMap<BlockPosition, Chunk>,
}

impl World {
  pub fn new() -> Self {
    Self {
      loaded_chunks: HashMap::new(),
    }
  }

  pub fn set_block(
    &mut self,
    position: BlockPosition,
    block: Option<Rc<Block>>,
  ) {
    let chunk_origin = position / 32;
    let relpos = position - chunk_origin;
    let chunk = match self.loaded_chunks.get_mut(&chunk_origin) {
      Some(chunk) => chunk,
      None => {
        self
          .loaded_chunks
          .insert(chunk_origin, Chunk::new(chunk_origin));
        self.loaded_chunks.get_mut(&chunk_origin).unwrap()
      }
    };
    chunk.set_block(relpos, block);
  }

  pub fn meshes(
    &mut self,
    registry: &BlockRegistry,
    camera: &Camera,
    device: &Device,
  ) -> Vec<&Mesh> {
    self
      .loaded_chunks
      .values_mut()
      .filter(|chunk| chunk.is_visible(camera))
      .fold(Vec::<&Mesh>::new(), |mut acc, chunk| {
        chunk.invalidate_all_meshes(registry, device);
        acc.extend(chunk.meshes());
        acc
      })
  }
}
