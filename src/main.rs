#![feature(inline_const)]

mod engine;

use engine::BloomEngine;

fn main() {
  let engine = pollster::block_on(BloomEngine::new("Bloom Engine Test"));
  engine.run();
}
