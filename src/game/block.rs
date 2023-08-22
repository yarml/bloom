use std::collections::HashMap;

use wgpu::{
  BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState,
  FrontFace, MultisampleState, PipelineLayout, PipelineLayoutDescriptor,
  PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
  RenderPipelineDescriptor, ShaderModuleDescriptor, VertexState,
};

use crate::engine::BloomEngine;

use super::world::BlockPosition;

pub struct BlockRenderer {
  pipeline_layout: PipelineLayout,
  render_pipelines: HashMap<String, RenderPipeline>,
}

pub struct Block<'a> {
  name: String,
  render_pipeline: &'a RenderPipeline,
}

pub struct BlockInstance<'a, 'b> {
  block: &'a Block<'b>,
  position: BlockPosition,
}

impl<'a> Block<'a> {
  fn new(
    name: &str,
    engine: &'a mut BloomEngine,
    shader_src: Option<&str>,
  ) -> Self {
    let block_renderer = &mut engine.block_renderer;
    let render_pipeline = block_renderer.shader_program(
      engine,
      shader_src.unwrap_or(include_str!("block_shader.wgsl")),
    );
    Self {
      name: String::from(name),
      render_pipeline,
    }
  }
}

impl BlockRenderer {
  pub fn new(device: &Device) -> Self {
    let pipeline_layout =
      device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("block_pipeline_layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
      });
    Self {
      pipeline_layout,
      render_pipelines: HashMap::new(),
    }
  }

  pub fn shader_program(
    &mut self,
    engine: &BloomEngine,
    shader_src: &str,
  ) -> &RenderPipeline {
    if self.render_pipelines.contains_key(shader_src) {
      self.render_pipelines.get(shader_src).unwrap()
    } else {
      let shader = engine.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("dede"),
        source: wgpu::ShaderSource::Wgsl(shader_src.into()),
      });

      let render_pipeline =
        engine
          .device
          .create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(
              format!("block_render_pipeline:{}", shader_src).as_str(),
            ),
            layout: Some(&self.pipeline_layout),
            vertex: VertexState {
              module: &shader,
              entry_point: "vs_main",
              buffers: &[],
            },
            fragment: Some(FragmentState {
              module: &shader,
              entry_point: "fs_main",
              targets: &[Some(ColorTargetState {
                format: engine.config.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
              })],
            }),
            primitive: PrimitiveState {
              topology: PrimitiveTopology::TriangleList,
              strip_index_format: None,
              front_face: FrontFace::Ccw,
              cull_mode: Some(Face::Back),
              polygon_mode: PolygonMode::Fill,
              unclipped_depth: false,
              conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
              count: 1,
              mask: !0,
              alpha_to_coverage_enabled: false,
            },
            multiview: None,
          });
      self
        .render_pipelines
        .insert(String::from(shader_src), render_pipeline);
      self.render_pipelines.get(shader_src).unwrap()
    }
  }
}
