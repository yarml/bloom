use image::GenericImageView;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  AddressMode, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingResource, Buffer, BufferUsages, Device,
  FilterMode, RenderPass, SamplerDescriptor, TextureViewDescriptor, ImageDataLayout, TextureAspect, Origin3d, ImageCopyTexture, TextureUsages, TextureDimension, TextureFormat, TextureDescriptor, Extent3d, Queue, BindGroupLayout, BindGroup,
};

use self::instance::BlockInstance;

pub mod instance;
pub mod model;
pub mod registry;

pub struct Block {
  pub name: String,

  instances: Vec<BlockInstance>,
  instances_buffer: Buffer,
  dirty: bool, // When instances and instances_buffer are not synchronized

  texture_bind_group: BindGroup,
}

impl Block {
  pub fn new(
    name: &str,
    texture_data: &[u8],
    instances: Vec<BlockInstance>,
    texture_bind_group_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Self {
    let instances_buffer =
      Self::create_instances_buffer(name, &instances, device);

    let texture_image = image::load_from_memory(texture_data).unwrap();
    let texture_rgba = texture_image.to_rgba8();
    let texture_dimensions = texture_image.dimensions();

    let texture_size = Extent3d {
      width: texture_dimensions.0,
      height: texture_dimensions.1,
      depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&TextureDescriptor {
      label: Some(format!("block/texture:{}", name).as_str()),
      size: texture_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Rgba8UnormSrgb,
      usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
      view_formats: &[],
    });
    queue.write_texture(
      ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: Origin3d::ZERO,
        aspect: TextureAspect::All,
      },
      &texture_rgba,
      ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * texture_dimensions.0),
        rows_per_image: Some(texture_dimensions.1),
      },
      texture_size,
    );

    let texture_view = texture.create_view(&TextureViewDescriptor::default());
    let texture_sampler = device.create_sampler(&SamplerDescriptor {
      label: Some(format!("block/texture/sampler:{}", name).as_str()),
      address_mode_u: AddressMode::MirrorRepeat,
      address_mode_v: AddressMode::MirrorRepeat,
      address_mode_w: AddressMode::MirrorRepeat,
      mag_filter: FilterMode::Nearest,
      min_filter: FilterMode::Nearest,
      mipmap_filter: FilterMode::Nearest,
      ..Default::default()
    });

    let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some(format!("block/texture/bind_group:{}", name).as_str()),
      layout: texture_bind_group_layout,
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: BindingResource::TextureView(&texture_view),
        },
        BindGroupEntry {
          binding: 1,
          resource: BindingResource::Sampler(&texture_sampler),
        },
      ],
    });

    Self {
      name: format!("block:{}", name),
      instances,
      instances_buffer,
      dirty: false,
      texture_bind_group,
    }
  }

  fn create_instances_buffer(
    name: &str,
    instances: &Vec<BlockInstance>,
    device: &Device,
  ) -> Buffer {
    let instances_data = instances
      .iter()
      .map(BlockInstance::model_matrix)
      .collect::<Vec<_>>();
    device.create_buffer_init(&BufferInitDescriptor {
      label: Some(format!("block/instances:{}", name).as_str()),
      contents: bytemuck::cast_slice(&instances_data),
      usage: BufferUsages::VERTEX,
    })
  }

  pub fn add_instance(&mut self, instance: BlockInstance) {
    self.instances.push(instance);
    self.dirty = true;
  }

  pub fn render_instances<'selftime>(
    &'selftime mut self,
    indices_count: u32,
    render_pass: &mut RenderPass<'selftime>,
    device: &Device,
  ) {
    if self.dirty {
      self.instances_buffer = Self::create_instances_buffer(
        self.name.as_str(),
        &self.instances,
        device,
      );
    }
    render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
    render_pass.set_vertex_buffer(1, self.instances_buffer.slice(..));
    render_pass.draw_indexed(0..indices_count, 0, 0..self.instances.len() as _);
  }

  const BIND_LAYOUT_ENTRIES: [BindGroupLayoutEntry; 2] = [
    BindGroupLayoutEntry {
      binding: 0,
      visibility: wgpu::ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Texture {
        multisampled: false,
        view_dimension: wgpu::TextureViewDimension::D2,
        sample_type: wgpu::TextureSampleType::Float { filterable: true },
      },
      count: None,
    },
    BindGroupLayoutEntry {
      binding: 1,
      visibility: wgpu::ShaderStages::FRAGMENT,
      // This should match the filterable field of the
      // corresponding Texture entry above.
      ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
      count: None,
    },
  ];
  pub fn texture_bind_group_layout(
    label: Option<&str>,
  ) -> BindGroupLayoutDescriptor {
    BindGroupLayoutDescriptor {
      label,
      entries: &Self::BIND_LAYOUT_ENTRIES,
    }
  }
}
