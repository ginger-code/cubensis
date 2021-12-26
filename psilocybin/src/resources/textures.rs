use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::scene::assets::{SceneTextures, TextureAsset};
use image::{GenericImageView, ImageFormat};
use psilocyn::device::GraphicsDevice;
use psilocyn::resources::CubensisResource;
use std::time::Duration;
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, TextureViewDescriptor};
use winit::event::Event;

pub struct TextureResource {
    graphics: std::rc::Rc<GraphicsDevice>,
    binding_group: u32,
    binding_offset: u32,
    textures: [wgpu::Texture; 5],
    texture_views: [wgpu::TextureView; 5],
    sampler: wgpu::Sampler,
}

impl TextureResource {
    pub fn new(
        graphics: std::rc::Rc<GraphicsDevice>,
        configuration: Configuration,
        binding_group: u32,
        binding_offset: u32,
    ) -> Self {
        let textures = configuration
            .library
            .build_library()
            .current_scene()
            .textures
            .clone();
        let textures = Self::create_textures(&graphics, &textures);
        let texture_views = Self::create_texture_views(&textures);
        let sampler = graphics.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..wgpu::SamplerDescriptor::default()
        });
        Self {
            graphics,
            binding_group,
            binding_offset,
            textures,
            texture_views,
            sampler,
        }
    }
    fn create_texture_views(textures: &[wgpu::Texture; 5]) -> [wgpu::TextureView; 5] {
        [
            textures[0].create_view(&TextureViewDescriptor::default()),
            textures[1].create_view(&TextureViewDescriptor::default()),
            textures[2].create_view(&TextureViewDescriptor::default()),
            textures[3].create_view(&TextureViewDescriptor::default()),
            textures[4].create_view(&TextureViewDescriptor::default()),
        ]
    }
    fn create_textures(
        graphics: &std::rc::Rc<GraphicsDevice>,
        textures: &SceneTextures,
    ) -> [wgpu::Texture; 5] {
        [
            Self::create_texture(graphics, &textures.texture_1),
            Self::create_texture(graphics, &textures.texture_2),
            Self::create_texture(graphics, &textures.texture_3),
            Self::create_texture(graphics, &textures.texture_4),
            Self::create_texture(graphics, &textures.texture_5),
        ]
    }
    fn create_texture(
        graphics: &std::rc::Rc<GraphicsDevice>,
        texture: &Option<TextureAsset>,
    ) -> wgpu::Texture {
        match texture {
            None => {
                log::debug!("Creating empty sampled texture");
                let texture_size = wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                };
                let rgb: [u8; 4] = [0, 0, 0, 0];
                let texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Asset Texture"),
                    size: texture_size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                });
                graphics.queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &rgb,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: std::num::NonZeroU32::new(4 * texture_size.width),
                        rows_per_image: std::num::NonZeroU32::new(texture_size.height),
                    },
                    texture_size,
                );
                texture
            }
            Some(ref asset) => {
                log::debug!("Creating sampled texture for image at path {}", asset.path);
                let bytes = std::fs::read(&asset.path).unwrap();
                log::debug!("Loaded image of {} bytes", bytes.len());
                // let format = image::guess_format(bytes.as_slice()).unwrap();
                let im = image::load_from_memory(bytes.as_slice()).unwrap();
                let dimensions = im.dimensions();
                let mut rgb = im.as_rgb8().unwrap().as_raw().clone();
                rgb.resize(256 * 256 * 256 * 4, 0); //todo: use some actual math here, lol
                let texture_size = wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                };
                log::debug!(
                    "Sampled texture size: {}x{}",
                    texture_size.width,
                    texture_size.height
                );
                let texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
                    size: texture_size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    label: Some("Asset Texture"),
                });
                log::debug!("Created sampled texture");
                log::debug!("Writing sampled texture data to GPU. {} bytes", rgb.len());
                graphics.queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    rgb.as_slice(),
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: std::num::NonZeroU32::new(4 * texture_size.width),
                        rows_per_image: std::num::NonZeroU32::new(texture_size.height),
                    },
                    texture_size,
                );
                log::debug!("Wrote sampled texture data to GPU");
                texture
            }
        }
    }
}

impl CubensisResource for TextureResource {
    fn update(&mut self, _: Duration) -> bool {
        false
    }

    fn resize(&mut self) {}

    fn get_bind_group_layout_entries(&self) -> Vec<BindGroupLayoutEntry> {
        vec![
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset + 1,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset + 2,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset + 3,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset + 4,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: self.binding_offset + 5,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                count: None,
            },
        ]
    }

    fn get_bind_group_entries(&self) -> Vec<BindGroupEntry> {
        vec![
            wgpu::BindGroupEntry {
                binding: self.binding_offset,
                resource: wgpu::BindingResource::TextureView(&self.texture_views[0]),
            },
            wgpu::BindGroupEntry {
                binding: self.binding_offset + 1,
                resource: wgpu::BindingResource::TextureView(&self.texture_views[1]),
            },
            wgpu::BindGroupEntry {
                binding: self.binding_offset + 2,
                resource: wgpu::BindingResource::TextureView(&self.texture_views[2]),
            },
            wgpu::BindGroupEntry {
                binding: self.binding_offset + 3,
                resource: wgpu::BindingResource::TextureView(&self.texture_views[3]),
            },
            wgpu::BindGroupEntry {
                binding: self.binding_offset + 4,
                resource: wgpu::BindingResource::TextureView(&self.texture_views[4]),
            },
            wgpu::BindGroupEntry {
                binding: self.binding_offset + 5,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            },
        ]
    }

    fn handle_or_capture_event(&mut self, _: &Event<'_, CubensisEvent>) -> bool {
        false
    }

    fn binding_group(&self) -> u32 {
        self.binding_group
    }

    fn binding_offset(&self) -> u32 {
        self.binding_offset
    }

    fn binding_size() -> u32 {
        6
    }
}
