use std::{collections::HashMap, rc::Rc};

use super::Block;

pub struct BlockRegistry {
  blocks: HashMap<String, Rc<Block>>,
}

impl BlockRegistry {
  pub fn new() -> Self {
    Self {
      blocks: HashMap::new(),
    }
  }

  pub fn register_block(&mut self, block: &Rc<Block>) {
    self.blocks.insert(block.name.clone(), Rc::clone(block));
  }

  pub fn find_block(&self, name: &str) -> Rc<Block> {
    Rc::clone(self.blocks.get(name).unwrap())
  }
}
