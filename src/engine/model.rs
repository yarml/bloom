use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
  tex_coords: [f32; 2],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
  pub fn layout() -> VertexBufferLayout<'static> {
    use std::mem;

    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }

  pub fn new(x: f32, y: f32, z: f32, tx_x: f32, tx_y: f32) -> Self {
    Self {
      position: [x, y, z],
      tex_coords: [tx_x, tx_y],
    }
  }
}

impl From<((f32, f32, f32), (f32, f32))> for Vertex {
  fn from(value: ((f32, f32, f32), (f32, f32))) -> Self {
    Vertex::new(value.0 .0, value.0 .1, value.0 .2, value.1 .0, value.1 .1)
  }
}
