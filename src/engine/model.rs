use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  position: [f32; 3],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
  pub fn layout() -> VertexBufferLayout<'static> {
    use std::mem;

    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}
