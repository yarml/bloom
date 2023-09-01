pub mod camera;
pub mod game;
pub mod math;
pub mod mesh;
pub mod model;
pub mod renderer;
pub mod texture;

use std::{collections::HashMap, rc::Rc, time::SystemTime};

use anyhow::*;
use cgmath::{Deg, Vector3};
use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
  event::{Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use self::{
  game::{block::registry::BlockRegistry, world::World},
  math::Orientation2,
  mesh::Mesh,
  renderer::BloomRenderer,
  texture::BloomTexture,
};

pub struct BloomEngine {
  pub renderer: BloomRenderer,
  pub event_loop: EventLoop<()>,
  pub window: Window,

  pub block_registry: BlockRegistry,
  pub world: World,

  textures: HashMap<String, Rc<BloomTexture>>,
  meshes: Vec<Mesh>,
}

impl BloomEngine {
  pub async fn new(win_title: &str) -> Self {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title(win_title);
    let window = wb.build(&event_loop).unwrap();

    let renderer = BloomRenderer::new(&window).await;

    let block_registry = BlockRegistry::new();
    let world = World::new();

    let (textures, meshes) = Self::init(
      &renderer.texture_bind_group_layout,
      &renderer.device,
      &renderer.queue,
    )
    .unwrap();

    Self {
      renderer,
      event_loop,
      window,
      block_registry,
      world,

      textures,
      meshes,
    }
  }

  pub fn init(
    texture_bind_group_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Result<(HashMap<String, Rc<BloomTexture>>, Vec<Mesh>)> {
    let mut textures = HashMap::new();

    textures.insert(
      String::from("stone"),
      Rc::new(BloomTexture::from_raw_rbga(
        "stone",
        include_bytes!("engine/game/textures/stone.png"),
        texture_bind_group_layout,
        device,
        queue,
      )?),
    );

    let stone_texture = textures.get("stone").unwrap().clone();
    let mut meshes = Vec::new();

    let test_mesh = Mesh::new(
      "test",
      &[
        // Front
        ((0.0, 0.0, 1.0), (0.0, 1.0)).into(), // 0
        ((1.0, 0.0, 1.0), (1.0, 1.0)).into(), // 1
        ((1.0, 1.0, 1.0), (1.0, 0.0)).into(), // 2
        ((0.0, 1.0, 1.0), (0.0, 0.0)).into(), // 3
        // Back
        ((0.0, 0.0, 0.0), (1.0, 1.0)).into(), // 4
        ((1.0, 0.0, 0.0), (0.0, 1.0)).into(), // 5
        ((1.0, 1.0, 0.0), (0.0, 0.0)).into(), // 6
        ((0.0, 1.0, 0.0), (1.0, 0.0)).into(), // 7
        // Top
        ((1.0, 1.0, 1.0), (1.0, 1.0)).into(), // 8 -> 2
        ((0.0, 1.0, 1.0), (0.0, 1.0)).into(), // 9 -> 3
        ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 10 -> 6
        ((0.0, 1.0, 0.0), (0.0, 0.0)).into(), // 11 -> 7
        // Bottom
        ((0.0, 0.0, 1.0), (0.0, 0.0)).into(), // 12 -> 0
        ((1.0, 0.0, 1.0), (1.0, 0.0)).into(), // 13 -> 1
        ((0.0, 0.0, 0.0), (0.0, 1.0)).into(), // 14 -> 4
        ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 15 -> 5
        // Positive side
        ((1.0, 0.0, 1.0), (0.0, 1.0)).into(), // 16 -> 1
        ((1.0, 1.0, 1.0), (0.0, 0.0)).into(), // 17 -> 2
        ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 18 -> 5
        ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 19 -> 6
        // Negative side
        ((0.0, 0.0, 1.0), (1.0, 1.0)).into(), // 20 -> 0
        ((0.0, 1.0, 1.0), (1.0, 0.0)).into(), // 21 -> 3
        ((0.0, 0.0, 0.0), (0.0, 1.0)).into(), // 22 -> 4
        ((0.0, 1.0, 0.0), (0.0, 0.0)).into(), // 23 -> 7
      ],
      &[
        0, 1, 2, 0, 2, 3, // Front
        5, 4, 7, 5, 7, 6, // Back
        9, 8, 10, 9, 10, 11, // Top
        14, 15, 13, 14, 13, 12, // Bottom
        16, 18, 19, 16, 19, 17, // Positive side
        22, 20, 21, 22, 21, 23, // Negative side
      ],
      stone_texture,
      device,
    );

    meshes.push(test_mesh);

    Ok((textures, meshes))
  }

  pub fn run(self) {
    let Self {
      mut renderer,
      event_loop,
      window,

      block_registry: _,
      world: _,

      textures: _,
      meshes: _,
    } = self;

    let mut last_frame_time = SystemTime::now();
    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
      let delta = last_frame_time.elapsed().unwrap().as_secs_f32();
      last_frame_time = SystemTime::now();

      input.update(&event);

      Self::update(delta, &input, &mut renderer);

      match event {
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(window_id) if window.id() == window_id => {
          renderer.render(&self.meshes).unwrap();
        }
        Event::WindowEvent {
          window_id,
          ref event,
        } if window_id == window.id() => match event {
          WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
          WindowEvent::Resized(physical_size) => {
            renderer.resize(*physical_size);
          }
          WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            renderer.resize(**new_inner_size);
          }
          _ => {}
        },
        _ => {}
      }
    })
  }

  fn update(
    delta: f32,
    input: &WinitInputHelper,
    renderer: &mut BloomRenderer,
  ) {
    let camera_speed = 7.0 * delta;
    let sensitivity = 120.0 * delta;

    let camera = &mut renderer.camera;

    let mut displacement: Vector3<f32> = (0.0, 0.0, 0.0).into();
    if input.key_held(VirtualKeyCode::W) {
      displacement -= camera.forward() * camera_speed;
    }
    if input.key_held(VirtualKeyCode::S) {
      displacement += camera.forward() * camera_speed;
    }

    if input.key_held(VirtualKeyCode::D) {
      displacement -= camera.left() * camera_speed;
    }
    if input.key_held(VirtualKeyCode::A) {
      displacement += camera.left() * camera_speed;
    }
    if input.key_held(VirtualKeyCode::Space) {
      displacement += camera.up() * camera_speed;
    }
    if input.key_held(VirtualKeyCode::LShift) {
      displacement -= camera.up() * camera_speed;
    }

    let mut delta_orientation: Orientation2 = (0.0, 0.0).into();
    if input.key_held(VirtualKeyCode::Up) {
      delta_orientation -= (0.0, sensitivity).into();
    }
    if input.key_held(VirtualKeyCode::Down) {
      delta_orientation += (0.0, sensitivity).into();
    }

    if input.key_held(VirtualKeyCode::Right) {
      delta_orientation += (sensitivity, 0.0).into();
    }
    if input.key_held(VirtualKeyCode::Left) {
      delta_orientation -= (sensitivity, 0.0).into();
    }

    if input.key_pressed(VirtualKeyCode::LControl) {
      println!("Camera: {}", camera);
    }

    if input.key_held(VirtualKeyCode::O) {
      camera.inc_fovy(Deg(10.0 * delta));
    }
    if input.key_held(VirtualKeyCode::P) {
      camera.inc_fovy(Deg(-10.0 * delta));
    }

    camera.displace(displacement);
    camera.rotate(delta_orientation);
  }
}
