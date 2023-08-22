mod engine;
mod game;

use engine::BloomEngine;
use winit::event_loop::EventLoop;

fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();
  let mut engine = pollster::block_on(BloomEngine::new("Bloom", &event_loop));
  event_loop.run(move |event, _, control_flow| {
    engine.process_event(event, control_flow)
  });
}
