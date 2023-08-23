use wgpu::{
  Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor,
  Features, Limits, Operations, PowerPreference, Queue,
  RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions,
  Surface, SurfaceConfiguration, SurfaceError, TextureUsages,
  TextureViewDescriptor,
};
use winit::window::Window;

pub struct BloomRenderer {
  pub surface: Surface,
  pub device: Device,
  pub queue: Queue,
  pub config: SurfaceConfiguration,
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

    Self {
      surface,
      queue,
      device,
      config,
    }
  }

  pub fn render(&self) -> Result<(), SurfaceError> {
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

    {
      let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
