use std::{
  collections::{hash_map::Values, HashMap},
  rc::Rc,
};

use strum::IntoEnumIterator;
use wgpu::Device;

use crate::engine::{
  camera::Camera,
  game::block::{
    instance::{BlockInstance, BlockPosition},
    model::BlockMeshLocation,
    registry::BlockRegistry,
    Block,
  },
  mesh::Mesh,
};

pub const CHUNK_DIMEN: usize = 32;
const CHUNK_BLOCK_COUNT: usize = CHUNK_DIMEN * CHUNK_DIMEN * CHUNK_DIMEN;

// A chunk is a 32x32x32 array of block instances
pub struct Chunk {
  origin: BlockPosition,
  blocks: [Option<BlockInstance>; CHUNK_BLOCK_COUNT],
  block_type_counts: HashMap<String, u32>, // Map block_name -> block_count
  meshes: HashMap<String, Mesh>,           // Map block_name -> mesh

  dirty: bool, // Set when a block update happens, cleared when all meshes are invalidated
}

impl Chunk {
  pub fn new(origin: BlockPosition) -> Self {
    Self {
      origin,
      blocks: [const { None }; CHUNK_BLOCK_COUNT],
      block_type_counts: HashMap::new(),
      meshes: HashMap::new(),
      dirty: false,
    }
  }

  fn block_count(&self, name: &str) -> u32 {
    match self.block_type_counts.get(name) {
      Some(block_count) => *block_count,
      None => 0,
    }
  }

  fn inc_block_count(&mut self, name: &str) {
    self
      .block_type_counts
      .insert(String::from(name), self.block_count(name) + 1);
  }

  fn dec_block_count(&mut self, name: &str) {
    let current_count = self.block_count(name);
    if current_count == 1 {
      self.block_type_counts.remove(name);
    } else {
      self
        .block_type_counts
        .insert(String::from(name), self.block_count(name) - 1);
    }
  }

  // pub fn origin(&self) -> BlockPosition {
  //   self.origin
  // }

  pub fn set_block(&mut self, relpos: BlockPosition, block: Option<Rc<Block>>) {
    if !relpos.is_valid_chunk_relpos() {
      return;
    }
    self.dirty = true;
    let block_idx = relpos.x as usize
      + relpos.y as usize * CHUNK_DIMEN
      + relpos.z as usize * CHUNK_DIMEN * CHUNK_DIMEN;
    if self.blocks[block_idx].is_some() {
      let block = self.blocks[block_idx].as_ref().unwrap();
      let target_name = String::from(block.block_type().name());
      self.dec_block_count(&target_name);
    }
    if block.is_some() {
      let block = block.as_ref().unwrap();
      let block_name = block.name();
      self.inc_block_count(block_name);
    }
    self.blocks[block_idx] = match block {
      Some(block) => Some(BlockInstance::new(block, self.origin + relpos)),
      None => None,
    }
  }

  pub fn block_at(&self, relpos: BlockPosition) -> Option<&BlockInstance> {
    if !relpos.is_valid_chunk_relpos() {
      return None;
    }

    self.blocks[relpos.x as usize
      + relpos.y as usize * CHUNK_DIMEN
      + relpos.z as usize * CHUNK_DIMEN * CHUNK_DIMEN]
      .as_ref()
  }
  pub fn block_at_abs(&self, abspos: BlockPosition) -> Option<&BlockInstance> {
    self.block_at(abspos - self.origin)
  }

  pub fn is_visible(&self, _camera: &Camera) -> bool {
    true
  }

  pub fn meshes(&self) -> Values<'_, String, Mesh> {
    self.meshes.values()
  }

  pub fn invalidate_all_meshes(
    &mut self,
    registry: &BlockRegistry,
    device: &Device,
  ) {
    if !self.dirty {
      return;
    }
    eprintln!("Invalidating meshes for chunk at {}", self.origin);
    self.dirty = false;
    self.meshes.clear();
    let block_type_counts = self.block_type_counts.clone(); // I hate this
    block_type_counts.keys().for_each(|block_name| {
      let block_type = registry.find_block(block_name);
      if self.block_count(block_name) == 0 {
        self.meshes.remove(block_name);
      }

      let mut vertices = Vec::new();
      let mut indices = Vec::new();

      self
        .blocks
        .iter()
        .filter(|block| block.is_some())
        .map(|block| block.as_ref().unwrap())
        .filter(|block| block.block_type().name() == block_name)
        .for_each(|block| {
          eprintln!("Block instance {}", block);
          let block_vertices = block
            .block_type()
            .model()
            .vertices_at(block.position().into());
          let indices_shift = vertices.len() as u16;
          vertices.extend(block_vertices);

          BlockMeshLocation::iter().for_each(|side| {
            let neighbour = self.block_at_abs(block.position().neighbour(side));
            if match neighbour {
              None => true,
              Some(neighbour_block)
                if side == BlockMeshLocation::Inside
                  || !neighbour_block
                    .block_type()
                    .model()
                    .has_face_at(side.opposite()) =>
              {
                true
              }
              Some(_) => false,
            } {
              eprintln!("Adding side {:?}", side);
              indices.extend(
                block.block_type().model().indices_of(side, indices_shift),
              );
            }
          })
        });
      let mesh = Mesh::new(
        format!("mesh:chunk({}):{}", self.origin, block_name).as_str(),
        &vertices,
        &indices,
        Rc::clone(&block_type.texture),
        device,
      );
      self.meshes.insert(String::from(block_name), mesh);
    });
  }
}
