use wgpu::{
  Backends, Device, InstanceDescriptor, PowerPreference, RequestAdapterOptions,
  Surface, SurfaceConfiguration,
};
use winit::{
  dpi::PhysicalSize,
  event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

use crate::game::block::BlockRenderer;

pub mod model;

pub struct BloomEngine {
  win: Window,
  surface: Surface,
  pub device: Device,
  pub config: SurfaceConfiguration,
  queue: wgpu::Queue,
  size: PhysicalSize<u32>,

  pub block_renderer: BlockRenderer,
}

impl BloomEngine {
  pub async fn new(win_title: &str, event_loop: &EventLoop<()>) -> Self {
    let wb = WindowBuilder::new().with_title(win_title);
    let win = wb.build(&event_loop).unwrap();

    let size = win.inner_size();

    let instance = wgpu::Instance::new(InstanceDescriptor {
      backends: Backends::VULKAN
        | Backends::GL
        | Backends::DX11
        | Backends::DX12,
      dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&win) }.unwrap();

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
        &wgpu::DeviceDescriptor {
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
          label: None,
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
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    let block_renderer = BlockRenderer::new(&device);

    Self {
      win,
      surface,
      size,
      queue,
      device,
      config,
      block_renderer,
    }
  }

  pub fn process_event(
    &mut self,
    event: Event<'_, ()>,
    control_flow: &mut ControlFlow,
  ) {
    match event {
      Event::RedrawRequested(window_id) if window_id == self.win.id() => {
        self.update();
        match self.render() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
          Err(wgpu::SurfaceError::OutOfMemory) => {
            *control_flow = ControlFlow::Exit
          }
          Err(e) => eprintln!("{:?}", e),
        }
      }
      Event::MainEventsCleared => {
        self.win.request_redraw();
      }
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == self.win.id() => match event {
        WindowEvent::Resized(physical_size) => {
          self.resize(*physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          self.resize(**new_inner_size);
        }
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: ElementState::Pressed,
              virtual_keycode: Some(VirtualKeyCode::Escape),
              ..
            },
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => {}
      },
      _ => {}
    }
  }

  fn render(&self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
      self
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
          label: Some("render_encoder"),
        });

    {
      let _render_pass =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("render_pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
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

  fn update(&self) {}

  fn resize(&mut self, new_size: PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }
}
