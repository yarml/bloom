pub mod instance;
pub mod model;
pub mod registry;

use std::rc::Rc;

use crate::engine::texture::BloomTexture;

use self::model::BlockModel;

pub struct Block {
  pub name: String,
  pub model: Rc<BlockModel>,
  pub texture: Rc<BloomTexture>,
}

impl Block {
  pub fn new(
    name: &str,
    model: Rc<BlockModel>,
    texture: Rc<BloomTexture>,
  ) -> Self {
    Self {
      name: name.into(),
      model,
      texture,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn model(&self) -> &BlockModel {
    &self.model
  }
}
