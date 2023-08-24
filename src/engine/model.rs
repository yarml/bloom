use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Buffer, BufferAddress, BufferUsages, Device, IndexFormat, RenderPass,
  VertexBufferLayout, VertexStepMode,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 1] =
    wgpu::vertex_attr_array![0 => Float32x3];
  pub fn layout() -> VertexBufferLayout<'static> {
    use std::mem;

    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      position: [x, y, z],
    }
  }
}

pub struct Model {
  vertex_buffer: Buffer,
  index_buffer: Buffer,
  indices_count: u32,
}

impl Model {
  pub fn new(vertices: &[Vertex], indices: &[u16], device: &Device) -> Self {
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("vertex_buffer"),
      contents: bytemuck::cast_slice(vertices),
      usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("index_buffer"),
      contents: bytemuck::cast_slice(indices),
      usage: BufferUsages::INDEX,
    });
    Self {
      vertex_buffer,
      index_buffer,
      indices_count: indices.len() as u32,
    }
  }

  pub fn render<'self_time>(
    &'self_time self,
    render_pass: &mut RenderPass<'self_time>,
  ) {
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass
      .set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
    render_pass.draw_indexed(0..self.indices_count, 0, 0..1);
  }
}

pub struct ModelStorage {
  models: Vec<Model>,
}

impl ModelStorage {
  pub fn new(models: Option<Vec<Model>>) -> Self {
    Self {
      models: models.unwrap_or(Vec::new()),
    }
  }

  pub fn add_model(
    &mut self,
    vertices: &[Vertex],
    indices: &[u16],
    device: &Device,
  ) {
    self.models.push(Model::new(vertices, indices, device));
  }

  pub fn iter(&self) -> std::slice::Iter<'_, Model> {
    self.models.iter()
  }
}
