use winit::{
  event::WindowEvent,
  window::{WindowAttributes, WindowId},
};

use crate::engine::BloomEngine;

pub struct BloomApp {
  title: &'static str,
  engine: Option<BloomEngine>,
}

impl BloomApp {
  pub fn new(title: &'static str) -> Self {
    Self {
      title,
      engine: None,
    }
  }
}

impl ApplicationHandler for BloomApp {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let attributes = WindowAttributes::default().with_title(self.title);
    let window = event_loop
      .create_window(attributes)
      .expect("Failed to create window");
    let engine = pollster::block_on(BloomEngine::new(window));
    self.engine = Some(engine);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::Resized(physical_size) => {
        if let Some(engine) = &mut self.engine {
          engine.renderer.resize(physical_size);
        }
      }
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      _ => {}
    }
  }
}
