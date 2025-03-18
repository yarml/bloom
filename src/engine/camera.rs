use std::fmt::Display;

use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3};
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
  BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
  BufferBindingType, BufferDescriptor, BufferUsages, Device, Queue,
  ShaderStages,
};

use super::math::{Orientation2, OPENGL_TO_WGPU_MATRIX};

pub struct Camera {
  position: Point3<f32>,
  orientation: Orientation2,

  fovy: Deg<f32>,
  aspect: f32,
  znear: f32,
  zfar: f32,

  camera_buffer: Buffer,
  pub camera_bind_group: BindGroup,
  // Save projection matrix each time it is rebuilt to avoid
  // unnecesarily sending it to the GPU each frame
  cached_proj_matrix: Option<[[f32; 4]; 4]>,
}

impl Camera {
  pub fn new(
    aspect: f32,
    camera_bind_group_layout: &BindGroupLayout,
    device: &Device,
  ) -> Self {
    let camera_buffer = device.create_buffer(&BufferDescriptor {
      label: Some("camera_buffer"),
      size: std::mem::size_of::<[[f32; 4]; 4]>() as u64,
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some("camera_bind_group"),
      layout: camera_bind_group_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    Self {
      position: (0.0, 2.0, 3.0).into(),
      orientation: (90.0, 45.0).into(),

      fovy: Deg(45.0),
      aspect,
      znear: 0.1,
      zfar: 100.0,

      camera_buffer,
      camera_bind_group,
      cached_proj_matrix: None,
    }
  }

  const BIND_LAYOUT_ENTRIES: [BindGroupLayoutEntry; 1] =
    [BindGroupLayoutEntry {
      count: None,
      binding: 0,
      visibility: ShaderStages::VERTEX,
      ty: BindingType::Buffer {
        ty: BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
      },
    }];
  pub fn bind_group_layout_desc(
    label: Option<&str>,
  ) -> BindGroupLayoutDescriptor {
    BindGroupLayoutDescriptor {
      label,
      entries: &Self::BIND_LAYOUT_ENTRIES,
    }
  }

  pub fn update_proj_matrix(&mut self, queue: &Queue) {
    let view =
      Matrix4::look_to_lh(self.position, self.forward(), Vector3::unit_y());
    let proj = perspective(self.fovy, self.aspect, self.znear, self.zfar);
    let matrix = OPENGL_TO_WGPU_MATRIX * proj * view;

    let buffer_content: [[f32; 4]; 4] = matrix.into();

    if self.cached_proj_matrix.is_none()
      || self.cached_proj_matrix.unwrap() != buffer_content
    {
      self.cached_proj_matrix = Some(buffer_content);

      queue.write_buffer(
        &self.camera_buffer,
        0,
        bytemuck::cast_slice(&[buffer_content]),
      );
    }
  }
  pub fn update_aspect(&mut self, aspect: f32) {
    self.aspect = aspect;
  }
  pub fn inc_fovy(&mut self, delta_fovy: Deg<f32>) {
    self.fovy += delta_fovy;
  }

  pub fn forward(&self) -> Vector3<f32> {
    self.orientation.direction()
  }
  pub fn up(&self) -> Vector3<f32> {
    Vector3::unit_y()
  }
  pub fn left(&self) -> Vector3<f32> {
    self.forward().cross(self.up()).normalize()
  }
  pub fn displace(&mut self, delta: Vector3<f32>) {
    self.position += delta;
  }
  pub fn rotate(&mut self, delta: Orientation2) {
    self.orientation += delta;
    if self.orientation.yaw > Deg(85.0) {
      self.orientation.yaw = Deg(85.0);
    } else if self.orientation.yaw < Deg(-85.0) {
      self.orientation.yaw = Deg(-85.0);
    }
  }

  // pub fn position(&self) -> Point3<f32> {
  //   self.position
  // }
}

impl Display for Camera {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let forward = self.forward();
    let left = self.left();
    write!(
      f,
      "position=(x={:.2},y={:.2},z={:.2}),orientation=(pitch={:.2?},yaw={:.2?}),fovy={:.2?},forward=(x={:.2},y={:.2},z={:.2}),forward_mag={:.2},left=(x={:.2},y={:.2},z={:.2}),left_mag={:.2}",
      self.position.x,
      self.position.y,
      self.position.z,
      self.orientation.pitch,
      self.orientation.yaw,
      self.fovy,
      forward.x,
      forward.y,
      forward.z,
      forward.magnitude(),
      left.x,
      left.y,
      left.z,
      left.magnitude(),
    )
  }
}
