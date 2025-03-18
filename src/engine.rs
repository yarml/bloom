pub mod camera;
pub mod game;
pub mod math;
pub mod mesh;
pub mod model;
pub mod renderer;
pub mod texture;

use std::{rc::Rc, time::SystemTime};

use crate::input::Input;

use self::{
  game::{
    block::{model::BlockModel, registry::BlockRegistry, Block},
    world::World,
  },
  math::Orientation2,
  renderer::BloomRenderer,
  texture::BloomTexture,
};
use anyhow::*;
use cgmath::{Deg, Vector3};
use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
  event::{Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

pub struct BloomEngine {
  pub renderer: BloomRenderer,
  pub event_loop: EventLoop<()>,
  pub window: Window,

  pub block_registry: BlockRegistry,
  pub world: World,
  // textures: HashMap<String, Rc<BloomTexture>>,
}

impl BloomEngine {
  pub async fn new(window: Window) -> Self {
    let event_loop = EventLoop::new();
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
    let simple_transparent_model =
      Rc::new(BlockModel::model_simple_transparent());

    let stone_block =
      Rc::new(Block::new("stone", &simple_model, &stone_texture));
    let oak_log_block =
      Rc::new(Block::new("oak_log", &side_vert_model, &oak_log_texture));
    let glass_block = Rc::new(Block::new(
      "glass",
      &simple_transparent_model,
      &glass_texture,
    ));

    let mut block_registry = BlockRegistry::new();
    block_registry.register_block(&stone_block);
    block_registry.register_block(&oak_log_block);
    block_registry.register_block(&glass_block);

    let mut world = World::new();
    world.set_block((1, 1, 1).into(), Some(&stone_block));
    world.set_block((1, 0, 1).into(), Some(&oak_log_block));

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
    let mut input = Input::new();
    event_loop.run(move |event, eloop| {
      let delta = last_frame_time.elapsed().unwrap().as_secs_f32();
      last_frame_time = SystemTime::now();

      input.update(&event);

      Self::update(delta, &input, &mut renderer, &mut world, &block_registry);

      match event {
        //Event::AboutToWait => window.request_redraw(),
        Event::WindowEvent {
          window_id,
          ref event,
        } if window_id == window.id() => match event {
          WindowEvent::RedrawRequested => {}
          WindowEvent::Resized(physical_size) => {
            renderer.resize(*physical_size);
          }
          WindowEvent::CloseRequested => {
            eloop.set_control_flow(ControlFlow::Exit)
          }
          _ => {}
        },
        _ => {}
      }
    });
  }

  pub fn redraw(&mut self) {
    let meshes = self.world.meshes(
      &self.block_registry,
      &self.renderer.camera,
      &self.renderer.device,
    );
    self.renderer.render(&meshes).unwrap();
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
      let block = block_registry.find_block("glass");
      world.set_block((1, 1, 2).into(), Some(&block));
    }
    if input.key_pressed(VirtualKeyCode::T) {
      let block = block_registry.find_block("glass");
      world.set_block((1, 1, 1).into(), Some(&block));
    }

    camera.displace(displacement);
    camera.rotate(delta_orientation);
  }
}

impl ApplicationHandler for BloomEngine {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    WindowAttributes::default().with_title(win_title)
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    todo!()
  }
}
