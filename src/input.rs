use std::collections::HashSet;
use winit::{
  event::{Event, WindowEvent},
  keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
  kbd: HashSet<KeyCode>,
}

impl Input {
  pub fn new() -> Self {
    Self {
      kbd: HashSet::new(),
    }
  }

  pub fn update(&mut self, event: &Event<()>) {
    match event {
      Event::WindowEvent {
        event: WindowEvent::KeyboardInput { event, .. },
        ..
      } => {
        if let PhysicalKey::Code(code) = event.physical_key {
          if event.state.is_pressed() {
            self.kbd.insert(code);
          } else {
            self.kbd.remove(&code);
          }
        }
      }
      _ => {}
    }
  }
}
