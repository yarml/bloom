use wgpu::{
  include_wgsl, Backends, BindGroupLayout, BlendState, Color, ColorTargetState,
  ColorWrites, CommandEncoderDescriptor, CompareFunction, DepthBiasState,
  DepthStencilState, Device, DeviceDescriptor, Extent3d, Face, Features,
  FragmentState, FrontFace, Limits, LoadOp, MultisampleState, Operations,
  PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState,
  PrimitiveTopology, Queue, RenderPassColorAttachment,
  RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
  RenderPipelineDescriptor, RequestAdapterOptions, StencilState, Surface,
  SurfaceConfiguration, SurfaceError, Texture, TextureDescriptor,
  TextureDimension, TextureFormat, TextureUsages, TextureView,
  TextureViewDescriptor, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use super::{
  camera::Camera,
  game::block::{
    instance::BlockInstance,
    registry::{Block, BlockRegistry},
  },
  model::Vertex,
};

pub struct BloomRenderer {
  pub surface: Surface,
  pub device: Device,
  pub queue: Queue,
  pub config: SurfaceConfiguration,
  pub size: PhysicalSize<u32>,
  pub camera: Camera,

  pub texture_bind_group_layout: BindGroupLayout,

  depth_texture_view: TextureView,
  depth_texture: Texture,

  default_render_pipeline: RenderPipeline,
}

impl BloomRenderer {
  pub async fn new(window: &Window) -> Self {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: Backends::all(),
      dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(window) }.unwrap();
    let adapter = instance
      .request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    let (device, queue) = adapter
      .request_device(
        &DeviceDescriptor {
          label: None,
          features: Features::empty(),
          limits: Limits::default(),
        },
        None,
      )
      .await
      .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .copied()
      .find(|f| f.is_srgb())
      .unwrap_or(surface_caps.formats[0]);
    let config = SurfaceConfiguration {
      usage: TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    let default_shader =
      device.create_shader_module(include_wgsl!("shaders/default.wgsl"));

    let camera_bind_group_layout = device.create_bind_group_layout(
      &Camera::bind_group_layout_desc(Some("camera_bind_group")),
    );
    let texture_bind_group_layout = device.create_bind_group_layout(
      &Block::texture_bind_group_layout(Some("block_texture_bind_group")),
    );

    let render_pipeline_layout =
      device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("render_pipeline_layout"),
        bind_group_layouts: &[
          &camera_bind_group_layout,
          &texture_bind_group_layout,
        ],
        push_constant_ranges: &[],
      });
    let default_render_pipeline =
      device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("render_pipeline:default"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
          module: &default_shader,
          entry_point: "vs_main",
          buffers: &[Vertex::layout(), BlockInstance::gfx_layout()],
        },
        fragment: Some(FragmentState {
          module: &default_shader,
          entry_point: "fs_main",
          targets: &[Some(ColorTargetState {
            format: config.format,
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
        depth_stencil: Some(DepthStencilState {
          format: TextureFormat::Depth32Float,
          depth_write_enabled: true,
          depth_compare: CompareFunction::Less,
          stencil: StencilState::default(),
          bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        },
        multiview: None,
      });
    let aspect_ratio = config.width as f32 / config.height as f32;
    let camera = Camera::new(aspect_ratio, &camera_bind_group_layout, &device);

    let (depth_texture, depth_texture_view) =
      Self::create_depth_texture(config.width, config.height, &device);

    Self {
      surface,
      queue,
      device,
      config,
      size,
      camera,

      texture_bind_group_layout,
      depth_texture,
      depth_texture_view,

      default_render_pipeline,
    }
  }

  fn create_depth_texture(
    width: u32,
    height: u32,
    device: &Device,
  ) -> (Texture, TextureView) {
    let depth_texture = device.create_texture(&TextureDescriptor {
      label: Some("depth_texture"),
      size: Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Depth32Float,
      usage: TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    });
    let depth_texture_view =
      depth_texture.create_view(&TextureViewDescriptor::default());
    (depth_texture, depth_texture_view)
  }

  pub fn render(
    &mut self,
    block_registry: &mut BlockRegistry,
  ) -> Result<(), SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&TextureViewDescriptor::default());

    let mut encoder =
      self
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
          label: Some("render_encoder"),
        });

    self.camera.update_proj_matrix(&self.queue);

    {
      let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("render_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: Operations {
            load: wgpu::LoadOp::Clear(Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
          view: &self.depth_texture_view,
          depth_ops: Some(Operations {
            load: LoadOp::Clear(1.0),
            store: true,
          }),
          stencil_ops: None,
        }),
      });
      render_pass.set_pipeline(&self.default_render_pipeline);
      render_pass.set_bind_group(0, &self.camera.camera_bind_group, &[]);

      block_registry.render(&mut render_pass, &self.device);
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);

      let new_aspect = new_size.width as f32 / new_size.height as f32;
      self.camera.update_aspect(new_aspect);

      let (depth_texture, depth_texture_view) = Self::create_depth_texture(
        new_size.width,
        new_size.height,
        &self.device,
      );
      self.depth_texture = depth_texture;
      self.depth_texture_view = depth_texture_view;
    }
  }
}
