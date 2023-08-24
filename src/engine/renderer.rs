use wgpu::{
  include_wgsl, Backends, BlendState, Color, ColorTargetState, ColorWrites,
  CommandEncoderDescriptor, Device, DeviceDescriptor, Face, Features,
  FragmentState, FrontFace, Limits, MultisampleState, Operations,
  PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState,
  PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
  RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, Surface,
  SurfaceConfiguration, SurfaceError, TextureUsages, TextureViewDescriptor,
  VertexState,
};
use winit::window::Window;

use super::{
  camera::Camera,
  model::{ModelStorage, Vertex},
};

pub struct BloomRenderer {
  pub surface: Surface,
  pub device: Device,
  pub queue: Queue,
  pub config: SurfaceConfiguration,
  pub camera: Camera,

  render_pipeline: RenderPipeline,
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

    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

    let camera_bind_group_layout = device.create_bind_group_layout(
      &Camera::bind_group_layout_desc(Some("camera_bind_group")),
    );

    let render_pipeline_layout =
      device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("render_pipeline_layout"),
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[],
      });
    let render_pipeline =
      device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("render_pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
          module: &shader,
          entry_point: "vs_main",
          buffers: &[Vertex::layout()],
        },
        fragment: Some(FragmentState {
          module: &shader,
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
        depth_stencil: None,
        multisample: MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        },
        multiview: None,
      });
    let aspect_ratio = config.width as f32 / config.height as f32;
    let camera = Camera::new(aspect_ratio, &camera_bind_group_layout, &device);

    Self {
      surface,
      queue,
      device,
      config,
      camera,

      render_pipeline,
    }
  }

  pub fn render(
    &mut self,
    model_storage: &ModelStorage,
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
        depth_stencil_attachment: None,
      });
      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.set_bind_group(0, &self.camera.camera_bind_group, &[]);

      model_storage
        .iter()
        .for_each(|model| model.render(&mut render_pass));
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
