mod engine;
mod input;
mod app;

use engine::BloomEngine;

fn main() {
  let engine = pollster::block_on(BloomEngine::new("Bloom Engine Test"));
  engine.run();
}
