use anyhow::*;
use image::{DynamicImage, GenericImageView};
use wgpu::{
  AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
  BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
  Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
  Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, Texture,
  TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
  TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor,
  TextureViewDimension,
};

#[derive(Debug)]
pub struct BloomTexture {
  pub texture: Texture,
  pub view: TextureView,
  pub sampler: Sampler,
  pub bind_group: BindGroup,
}

impl BloomTexture {
  pub fn from_raw_rbga(
    label: &str,
    raw: &[u8],
    bind_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Result<Self> {
    let img = image::load_from_memory(raw)?;
    Self::from_img(label, &img, bind_layout, device, queue)
  }
  pub fn from_img(
    label: &str,
    img: &DynamicImage,
    bind_layout: &BindGroupLayout,
    device: &Device,
    queue: &Queue,
  ) -> Result<Self> {
    let raw_rgba = img.to_rgba8();
    let dimen = img.dimensions();

    let size = Extent3d {
      width: dimen.0,
      height: dimen.1,
      depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&TextureDescriptor {
      label: Some(label),
      size,
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
      &raw_rgba,
      ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimen.0),
        rows_per_image: Some(dimen.1),
      },
      size,
    );

    let view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
      address_mode_u: AddressMode::Repeat,
      address_mode_v: AddressMode::Repeat,
      address_mode_w: AddressMode::Repeat,
      mag_filter: FilterMode::Nearest,
      min_filter: FilterMode::Nearest,
      mipmap_filter: FilterMode::Nearest,
      ..Default::default()
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some(label),
      layout: bind_layout,
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&view),
        },
        BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&sampler),
        },
      ],
    });

    Ok(Self {
      texture,
      view,
      sampler,
      bind_group,
    })
  }

  const BIND_LAYOUT_ENTRIES: [BindGroupLayoutEntry; 2] = [
    BindGroupLayoutEntry {
      binding: 0,
      visibility: ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Texture {
        multisampled: false,
        view_dimension: TextureViewDimension::D2,
        sample_type: TextureSampleType::Float { filterable: true },
      },
      count: None,
    },
    BindGroupLayoutEntry {
      binding: 1,
      visibility: ShaderStages::FRAGMENT,
      ty: BindingType::Sampler(SamplerBindingType::Filtering),
      count: None,
    },
  ];
  pub fn bind_group_layout() -> BindGroupLayoutDescriptor<'static> {
    BindGroupLayoutDescriptor {
      label: Some("texture_bind_group_layout"),
      entries: &Self::BIND_LAYOUT_ENTRIES,
    }
  }
}
