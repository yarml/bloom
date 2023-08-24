use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Buffer, BufferAddress, BufferUsages, Device, RenderPass, VertexBufferLayout,
  VertexStepMode,
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
  vertices_count: u32,
}

impl Model {
  pub fn new(vertices: &[Vertex], device: &Device) -> Self {
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("vertex_buffer"),
      contents: bytemuck::cast_slice(vertices),
      usage: BufferUsages::VERTEX,
    });
    Self {
      vertex_buffer,
      vertices_count: vertices.len() as u32,
    }
  }

  pub fn render<'self_time>(
    &'self_time self,
    render_pass: &mut RenderPass<'self_time>,
  ) {
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass.draw(0..self.vertices_count, 0..1);
  }
}
