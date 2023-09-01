use std::rc::Rc;

use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use super::{model::Vertex, texture::BloomTexture};

pub struct Mesh {
  vertex_buffer: Buffer,
  index_buffer: Buffer,
  indices_count: u32,

  texture: Rc<BloomTexture>,
}

impl Mesh {
  pub fn new(
    label: &str,
    vertices: &[Vertex],
    indices: &[u16],
    texture: Rc<BloomTexture>,
    device: &Device,
  ) -> Self {
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some(label),
      contents: bytemuck::cast_slice(vertices),
      usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some(label),
      contents: bytemuck::cast_slice(indices),
      usage: BufferUsages::INDEX,
    });
    Self {
      vertex_buffer,
      index_buffer,
      indices_count: indices.len() as u32,

      texture,
    }
  }

  pub fn render<'selftime>(
    &'selftime self,
    render_pass: &mut RenderPass<'selftime>,
  ) {
    render_pass.set_bind_group(1, &self.texture.bind_group, &[]);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass
      .set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
    render_pass.draw_indexed(0..self.indices_count, 0, 0..1);
  }
}
