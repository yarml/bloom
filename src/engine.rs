pub mod camera;
pub mod math;
pub mod model;
pub mod renderer;

use cgmath::Vector3;
use winit::{
  event::{Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use self::{
  math::Orientation2,
  model::{Model, ModelStorage, Vertex},
  renderer::BloomRenderer,
};

pub struct BloomEngine {
  pub renderer: BloomRenderer,
  pub event_loop: EventLoop<()>,
  pub window: Window,

  pub model_storage: ModelStorage,
}

impl BloomEngine {
  pub async fn new(win_title: &str) -> Self {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title(win_title);
    let window = wb.build(&event_loop).unwrap();

    let renderer = BloomRenderer::new(&window).await;

    let vertex_table: Vec<[Vertex; 4]> = (0..10)
      .map(|n| {
        [
          Vertex::new(0.5, 0.5, -n as f32 * 1.0),
          Vertex::new(0.0, 0.5, -n as f32 * 1.0),
          Vertex::new(0.5, 0.0, -n as f32 * 1.0),
          Vertex::new(0.5, 0.5, -n as f32 * 1.0 - 0.5),
        ]
      })
      .collect();

    let models: Vec<Model> = vertex_table
      .iter()
      .map(|vertices| {
        Model::new(
          vertices,
          &[0, 1, 2, 0, 3, 1, 1, 3, 2, 0, 2, 3],
          &renderer.device,
        )
      })
      .collect();

    let model_storage = ModelStorage::new(Some(models));

    Self {
      renderer,
      event_loop,
      window,
      model_storage,
    }
  }

  pub fn run(self) {
    let Self {
      mut renderer,
      event_loop,
      window,
      model_storage,
    } = self;

    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
      input.update(&event);

      Self::update(&input, &mut renderer);

      match event {
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(window_id) if window.id() == window_id => {
          renderer.render(&model_storage).unwrap();
        }
        Event::WindowEvent {
          window_id,
          ref event,
        } if window_id == window.id() => match event {
          WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
          _ => {}
        },
        _ => {}
      }
    })
  }

  fn update(input: &WinitInputHelper, renderer: &mut BloomRenderer) {
    let camera_speed = 0.003;
    let sensitivity = 0.1;

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

    let mut delta_orientation: Orientation2 = (0.0, 0.0).into();
    if input.key_held(VirtualKeyCode::Up) {
      delta_orientation += (0.0, sensitivity).into();
    }
    if input.key_held(VirtualKeyCode::Down) {
      delta_orientation -= (0.0, sensitivity).into();
    }

    if input.key_held(VirtualKeyCode::Right) {
      delta_orientation += (sensitivity, 0.0).into();
    }
    if input.key_held(VirtualKeyCode::Left) {
      delta_orientation -= (sensitivity, 0.0).into();
    }

    if input.key_pressed(VirtualKeyCode::Space) {
      println!("Camera: {}", camera);
    }

    camera.displace(displacement);
    camera.rotate(delta_orientation);
  }
}
