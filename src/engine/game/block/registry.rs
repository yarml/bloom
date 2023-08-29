use std::collections::HashMap;

use wgpu::{Device, RenderPass};

use super::{model::BlockModel, Block};

pub struct BlockRegistry {
  models: HashMap<String, BlockModel>,
}

impl BlockRegistry {
  pub fn new() -> Self {
    Self {
      models: HashMap::new(),
    }
  }

  pub fn register_model(&mut self, model: BlockModel) -> Result<(), ()> {
    if self.models.contains_key(&model.name) {
      Err(())
    } else {
      self.models.insert(model.name.clone(), model);
      Ok(())
    }
  }
  pub fn register_block(
    &mut self,
    model_name: &str,
    block: Block,
  ) -> Result<(), ()> {
    match self.find_model(model_name) {
      Some(model) => model.add_block(block),
      None => {
        eprintln!("model '{}' not found", model_name);
        Err(())
      }
    }
  }

  pub fn find_model(&mut self, name: &str) -> Option<&mut BlockModel> {
    self.models.get_mut(name)
  }
  pub fn find_block(&mut self, model: &str, name: &str) -> Option<&mut Block> {
    match self.models.get_mut(model) {
      None => None,
      Some(model) => model.find_block(name),
    }
  }

  pub fn render<'selftime>(
    &'selftime mut self,
    render_pass: &mut RenderPass<'selftime>,
    device: &Device,
  ) {
    self
      .models
      .values_mut()
      .for_each(|model| model.render_blocks(render_pass, device));
  }
}
