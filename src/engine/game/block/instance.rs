use std::mem::size_of;

use cgmath::{Matrix4, Vector3};
use wgpu::{BufferAddress, VertexBufferLayout};

#[derive(Clone, Copy)]
pub struct BlockPosition {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

pub struct BlockInstance {
  position: BlockPosition,
}

impl BlockInstance {
  pub fn new(position: BlockPosition) -> Self {
    Self { position }
  }

  pub fn model_matrix(&self) -> [[f32; 4]; 4] {
    Matrix4::from_translation(self.position.into()).into()
  }

  pub fn gfx_layout() -> VertexBufferLayout<'static> {
    use std::mem;
    VertexBufferLayout {
      array_stride: size_of::<[[f32; 4]; 4]>() as BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
      ],
    }
  }
}

impl Into<Vector3<f32>> for BlockPosition {
  fn into(self) -> Vector3<f32> {
    Vector3 {
      x: self.x as f32,
      y: self.y as f32,
      z: self.z as f32,
    }
  }
}

impl From<(i32, i32, i32)> for BlockPosition {
  fn from(value: (i32, i32, i32)) -> Self {
    Self {
      x: value.0,
      y: value.1,
      z: value.2,
    }
  }
}
