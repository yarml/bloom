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
}
