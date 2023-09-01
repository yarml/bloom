use image::GenericImageView;
use wgpu::{
  AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
  BindingResource, Device, Extent3d, FilterMode, ImageCopyTexture,
  ImageDataLayout, Origin3d, Queue, SamplerDescriptor, TextureAspect,
  TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
  TextureViewDescriptor,
};

pub mod instance;
pub mod registry;

pub struct Block {
  name: String,
  texture_bind_group: BindGroup,
}

impl Block {
  pub fn new(
    name: &str,
    texture_data: &[u8],
    texture_bind_group_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Self {
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
      texture_bind_group,
    }
  }
}
