pub mod renderer;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

use self::renderer::BloomRenderer;

pub struct BloomEngine {
  renderer: BloomRenderer,
  event_loop: EventLoop<()>,
  window: Window,
}

impl BloomEngine {
  pub async fn new(win_title: &str) -> Self {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title(win_title);
    let window = wb.build(&event_loop).unwrap();

    let renderer = BloomRenderer::new(&window).await;

    Self {
      renderer,
      event_loop,
      window,
    }
  }

  pub fn run(self) {
    self
      .event_loop
      .run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => self.window.request_redraw(),
        Event::RedrawRequested(window_id) if self.window.id() == window_id => {
          self.renderer.render().unwrap();
        }
        Event::WindowEvent {
          window_id,
          ref event,
        } if window_id == self.window.id() => match event {
          WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
          _ => {}
        },
        _ => {}
      })
  }
}
