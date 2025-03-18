use std::ops::{AddAssign, SubAssign};

use cgmath::{Deg, Rad, Vector3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Orientation2 {
  pub pitch: Deg<f32>, // Rotation around x axis
  pub yaw: Deg<f32>,   // Rotation around y axis
}

impl Orientation2 {
  pub fn new(pitch: Deg<f32>, yaw: Deg<f32>) -> Self {
    Self { pitch, yaw }
  }

  pub fn direction(&self) -> Vector3<f32> {
    let rad_pitch: Rad<f32> = self.pitch.into();
    let rad_yaw: Rad<f32> = self.yaw.into();
    let (sin_pitch, cos_pitch) = rad_pitch.0.sin_cos();
    let (sin_yaw, cos_yaw) = rad_yaw.0.sin_cos();
    Vector3 {
      x: cos_yaw * cos_pitch,
      y: sin_yaw,
      z: cos_yaw * sin_pitch,
    }
  }
}

impl From<(f32, f32)> for Orientation2 {
  fn from(value: (f32, f32)) -> Self {
    let pitch = value.0 % 360.0;
    let yaw = value.1 % 360.0;
    Orientation2::new(Deg(pitch), Deg(yaw))
  }
}

impl AddAssign for Orientation2 {
  fn add_assign(&mut self, rhs: Self) {
    self.yaw += rhs.yaw;
    self.pitch += rhs.pitch;

    self.yaw %= Deg(360.0);
    self.pitch %= Deg(360.0);
  }
}

impl SubAssign for Orientation2 {
  fn sub_assign(&mut self, rhs: Self) {
    self.yaw -= rhs.yaw;
    self.pitch -= rhs.pitch;

    self.yaw %= Deg(360.0);
    self.pitch %= Deg(360.0);
  }
}
