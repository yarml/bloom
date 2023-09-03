pub mod camera;
pub mod game;
pub mod math;
pub mod mesh;
pub mod model;
pub mod renderer;
pub mod texture;

use std::{rc::Rc, time::SystemTime};

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
  game::{
    block::{model::BlockModel, registry::BlockRegistry, Block},
    world::World,
  },
  math::Orientation2,
  renderer::BloomRenderer,
  texture::BloomTexture,
};

pub struct BloomEngine {
  pub renderer: BloomRenderer,
  pub event_loop: EventLoop<()>,
  pub window: Window,

  pub block_registry: BlockRegistry,
  pub world: World,
  // textures: HashMap<String, Rc<BloomTexture>>,
}

impl BloomEngine {
  pub async fn new(win_title: &str) -> Self {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title(win_title);
    let window = wb.build(&event_loop).unwrap();

    let renderer = BloomRenderer::new(&window).await;

    let (block_registry, world) = Self::init(
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
    }
  }

  pub fn init(
    texture_bind_group_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Result<(BlockRegistry, World)> {
    let stone_texture = Rc::new(BloomTexture::from_raw_rbga(
      "stone",
      include_bytes!("engine/game/textures/stone.png"),
      texture_bind_group_layout,
      device,
      queue,
    )?);
    let glass_texture = Rc::new(BloomTexture::from_raw_rbga(
      "glass",
      include_bytes!("engine/game/textures/glass.png"),
      texture_bind_group_layout,
      device,
      queue,
    )?);
    let oak_log_texture = Rc::new(BloomTexture::from_raw_rbga(
      "oak_log",
      include_bytes!("engine/game/textures/oak_log.png"),
      texture_bind_group_layout,
      device,
      queue,
    )?);

    let simple_model = Rc::new(BlockModel::model_simple());
    let side_vert_model = Rc::new(BlockModel::model_side_vert());

    let stone_block =
      Rc::new(Block::new("stone", &simple_model, &stone_texture));
    let oak_log_block =
      Rc::new(Block::new("oak_log", &side_vert_model, &oak_log_texture));
    let glass_block =
      Rc::new(Block::new("glass", &simple_model, &glass_texture));

    let mut block_registry = BlockRegistry::new();
    block_registry.register_block(&stone_block);
    block_registry.register_block(&oak_log_block);
    block_registry.register_block(&glass_block);

    let mut world = World::new();
    world.set_block((1, 1, 1).into(), Some(&stone_block));
    world.set_block((1, 0, 1).into(), Some(&stone_block));

    Ok((block_registry, world))
  }

  pub fn run(self) {
    let Self {
      mut renderer,
      event_loop,
      window,

      block_registry,
      mut world,
    } = self;

    let mut last_frame_time = SystemTime::now();
    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
      let delta = last_frame_time.elapsed().unwrap().as_secs_f32();
      last_frame_time = SystemTime::now();

      input.update(&event);

      Self::update(delta, &input, &mut renderer, &mut world, &block_registry);

      match event {
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(window_id) if window.id() == window_id => {
          let meshes =
            world.meshes(&block_registry, &renderer.camera, &renderer.device);
          renderer.render(&meshes).unwrap();
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
    world: &mut World,
    block_registry: &BlockRegistry,
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
    if input.key_held(VirtualKeyCode::I) {
      delta_orientation -= (0.0, sensitivity).into();
    }
    if input.key_held(VirtualKeyCode::K) {
      delta_orientation += (0.0, sensitivity).into();
    }

    if input.key_held(VirtualKeyCode::L) {
      delta_orientation += (sensitivity, 0.0).into();
    }
    if input.key_held(VirtualKeyCode::J) {
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

    if input.key_pressed(VirtualKeyCode::R) {
      let oak_log_block = block_registry.find_block("oak_log");
      world.set_block((1, 1, 2).into(), Some(&oak_log_block));
    }
    if input.key_pressed(VirtualKeyCode::T) {
      let glass_block = block_registry.find_block("glass");
      world.set_block((1, 1, 1).into(), Some(&glass_block));
    }

    camera.displace(displacement);
    camera.rotate(delta_orientation);
  }
}
