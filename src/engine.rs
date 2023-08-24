pub mod model;
pub mod renderer;

use winit::{
  event::{Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use self::{
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

    let model_storage = ModelStorage::new(Some(vec![
      Model::new(
        &[
          Vertex::new(-0.0868241, 0.49240386, 0.0),
          Vertex::new(-0.49513406, 0.06958647, 0.0),
          Vertex::new(-0.21918549, -0.44939706, 0.0),
          Vertex::new(0.35966998, -0.3473291, 0.0),
          Vertex::new(0.44147372, 0.2347359, 0.0),
        ],
        &[0, 1, 4, 1, 2, 4, 2, 3, 4],
        &renderer.device,
      ),
      Model::new(
        &[
          Vertex::new(0.5, 0.5, 0.0),
          Vertex::new(0.0, 0.5, 0.0),
          Vertex::new(0.5, 0.0, 0.0),
        ],
        &[0, 1, 2],
        &renderer.device,
      ),
    ]));

    Self {
      renderer,
      event_loop,
      window,
      model_storage,
    }
  }

  pub fn run(self) {
    let Self {
      renderer,
      event_loop,
      window,
      model_storage,
    } = self;

    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
      input.update(&event);

      Self::update(&input);

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

  fn update(input: &WinitInputHelper) {
    if input.key_pressed(VirtualKeyCode::Space) {
      println!("Space");
    }
  }
}
