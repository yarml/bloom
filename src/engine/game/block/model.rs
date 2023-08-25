use std::collections::HashMap;

use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use crate::engine::model::Vertex;

use super::registry::Block;

pub struct BlockModel {
  pub name: String,
  vertex_buffer: Buffer,
  index_buffer: Buffer,
  indices_count: u32,

  blocks: HashMap<String, Block>,
}

impl BlockModel {
  pub fn new(
    name: &str,
    vertices: &[Vertex],
    indices: &[u16],
    device: &Device,
  ) -> Self {
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some(format!("block/model/vertex:{}", name).as_str()),
      contents: bytemuck::cast_slice(vertices),
      usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some(format!("block/model/index:{}", name).as_str()),
      contents: bytemuck::cast_slice(indices),
      usage: BufferUsages::INDEX,
    });

    Self {
      name: format!("block/model:{}", name),
      vertex_buffer,
      index_buffer,
      indices_count: indices.len() as u32,

      blocks: HashMap::new(),
    }
  }
  pub fn add_block(&mut self, block: Block) -> Result<(), ()> {
    if self.blocks.contains_key(&block.name) {
      Err(())
    } else {
      self.blocks.insert(block.name.clone(), block);
      Ok(())
    }
  }
  pub fn find_block(&mut self, name: &str) -> Option<&mut Block> {
    self.blocks.get_mut(name)
  }

  pub fn render_blocks<'selftime>(
    &'selftime mut self,
    render_pass: &mut RenderPass<'selftime>,
    device: &Device,
  ) {
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass
      .set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
    self.blocks.values_mut().for_each(|block| {
      block.render_instances(self.indices_count, render_pass, device)
    });
  }
}
