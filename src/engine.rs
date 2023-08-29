pub mod camera;
pub mod game;
pub mod math;
pub mod model;
pub mod renderer;

use std::time::SystemTime;

use cgmath::Vector3;
use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
  event::{Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use self::{
  game::block::{
    instance::BlockInstance,
    model::BlockModel,
    registry::{Block, BlockRegistry},
  },
  math::Orientation2,
  renderer::BloomRenderer,
};

pub struct BloomEngine {
  pub renderer: BloomRenderer,
  pub event_loop: EventLoop<()>,
  pub window: Window,

  pub block_registry: BlockRegistry,
}

impl BloomEngine {
  pub async fn new(win_title: &str) -> Self {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title(win_title);
    let window = wb.build(&event_loop).unwrap();

    let renderer = BloomRenderer::new(&window).await;

    let mut block_registry = BlockRegistry::new();
    Self::register_models(&mut block_registry, &renderer.device);
    Self::register_blocks(
      &mut block_registry,
      &renderer.texture_bind_group_layout,
      &renderer.device,
      &renderer.queue,
    );
    Self::create_world(&mut block_registry);

    Self {
      renderer,
      event_loop,
      window,
      block_registry,
    }
  }

  pub fn run(self) {
    let Self {
      mut renderer,
      event_loop,
      window,
      mut block_registry,
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
          renderer.render(&mut block_registry).unwrap();
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

  fn register_models(block_registry: &mut BlockRegistry, device: &Device) {
    block_registry
      .register_model(BlockModel::new(
        "simple",
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
        device,
      ))
      .unwrap();

    block_registry
      .register_model(BlockModel::new(
        "side_vert",
        &[
          // Front
          ((0.0, 0.0, 1.0), (0.0, 0.5)).into(), // 0
          ((1.0, 0.0, 1.0), (0.5, 0.5)).into(), // 1
          ((1.0, 1.0, 1.0), (0.5, 0.0)).into(), // 2
          ((0.0, 1.0, 1.0), (0.0, 0.0)).into(), // 3
          // Back
          ((0.0, 0.0, 0.0), (0.5, 0.5)).into(), // 4
          ((1.0, 0.0, 0.0), (0.0, 0.5)).into(), // 5
          ((1.0, 1.0, 0.0), (0.0, 0.0)).into(), // 6
          ((0.0, 1.0, 0.0), (0.5, 0.0)).into(), // 7
          // Top
          ((1.0, 1.0, 1.0), (1.0, 1.0)).into(), // 8 -> 2
          ((0.0, 1.0, 1.0), (0.5, 1.0)).into(), // 9 -> 3
          ((1.0, 1.0, 0.0), (1.0, 0.0)).into(), // 10 -> 6
          ((0.0, 1.0, 0.0), (0.5, 0.0)).into(), // 11 -> 7
          // Bottom
          ((0.0, 0.0, 1.0), (0.5, 0.0)).into(), // 12 -> 0
          ((1.0, 0.0, 1.0), (1.0, 0.0)).into(), // 13 -> 1
          ((0.0, 0.0, 0.0), (0.5, 1.0)).into(), // 14 -> 4
          ((1.0, 0.0, 0.0), (1.0, 1.0)).into(), // 15 -> 5
          // Positive side
          ((1.0, 0.0, 1.0), (0.0, 0.5)).into(), // 16 -> 1
          ((1.0, 1.0, 1.0), (0.0, 0.0)).into(), // 17 -> 2
          ((1.0, 0.0, 0.0), (0.5, 0.5)).into(), // 18 -> 5
          ((1.0, 1.0, 0.0), (0.5, 0.0)).into(), // 19 -> 6
          // Negative side
          ((0.0, 0.0, 1.0), (0.5, 0.5)).into(), // 20 -> 0
          ((0.0, 1.0, 1.0), (0.5, 0.0)).into(), // 21 -> 3
          ((0.0, 0.0, 0.0), (0.0, 0.5)).into(), // 22 -> 4
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
        device,
      ))
      .unwrap();
  }

  fn register_blocks(
    block_registry: &mut BlockRegistry,
    texture_bind_group_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) {
    block_registry
      .register_block(
        "block/model:simple",
        Block::new(
          "stone",
          include_bytes!("engine/game/textures/stone.png"),
          vec![],
          texture_bind_group_layout,
          device,
          queue,
        ),
      )
      .unwrap();
    block_registry
      .register_block(
        "block/model:simple",
        Block::new(
          "stone_bricks",
          include_bytes!("engine/game/textures/stone_bricks.png"),
          vec![],
          texture_bind_group_layout,
          device,
          queue,
        ),
      )
      .unwrap();
    block_registry
      .register_block(
        "block/model:simple",
        Block::new(
          "glass",
          include_bytes!("engine/game/textures/glass.png"),
          vec![],
          texture_bind_group_layout,
          device,
          queue,
        ),
      )
      .unwrap();
    block_registry
      .register_block(
        "block/model:side_vert",
        Block::new(
          "oak_log",
          include_bytes!("engine/game/textures/oak_log.png"),
          vec![],
          texture_bind_group_layout,
          device,
          queue,
        ),
      )
      .unwrap();
  }

  fn create_world(block_registry: &mut BlockRegistry) {
    let glass = block_registry
      .find_block("block/model:simple", "block:glass")
      .unwrap();
    glass.add_instance(BlockInstance::new((0, 1, 0).into()));
    glass.add_instance(BlockInstance::new((2, 1, 0).into()));
    let stone_block = block_registry
      .find_block("block/model:simple", "block:stone")
      .unwrap();
    stone_block.add_instance(BlockInstance::new((0, 0, 0).into()));
    stone_block.add_instance(BlockInstance::new((2, 0, 0).into()));

    let stone_bricks_block = block_registry
      .find_block("block/model:simple", "block:stone_bricks")
      .unwrap();
    stone_bricks_block.add_instance(BlockInstance::new((1, 0, 0).into()));

    let oak_log = block_registry
      .find_block("block/model:side_vert", "block:oak_log")
      .unwrap();
    oak_log.add_instance(BlockInstance::new((1, 1, 0).into()));
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

    camera.displace(displacement);
    camera.rotate(delta_orientation);
  }
}
