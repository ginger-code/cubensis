use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use psilocyn::device::GraphicsDevice;
use psilocyn::resources::CubensisResource;
use std::time::Duration;
use substrate::wave_stream::WaveStream;
use substrate::AudioStreamSource;

pub struct AudioResource<const BIND_GROUP: u32, const BIND_OFFSET: u32> {
    graphics: std::rc::Rc<GraphicsDevice>,
    wave_stream: WaveStream,
    wave_texture_width: u32,
    spectrum_texture_width: u32,
    wave_texture: wgpu::Texture,
    spectrum_texture: wgpu::Texture,
    wave_view: wgpu::TextureView,
    spectrum_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl<const BIND_GROUP: u32, const BIND_OFFSET: u32> AudioResource<BIND_GROUP, BIND_OFFSET> {
    pub fn new(graphics: std::rc::Rc<GraphicsDevice>, configuration: Configuration) -> Self {
        log::debug!("Creating audio buffer resource");
        let stream_source = AudioStreamSource::default_stream();
        let mut wave_stream = WaveStream::new(stream_source, configuration.clone());
        let (wave_data, spectrum_data) = wave_stream.get_wave_and_spectrum_data();
        let wave_texture = Self::create_1d_texture(&graphics, wave_data, "Wave Data Texture");
        let spectrum_texture =
            Self::create_1d_texture(&graphics, spectrum_data, "Spectrum Data Texture");
        let wave_view = wave_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let spectrum_view = spectrum_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let wave_texture_width = wave_data.len() as u32;
        let spectrum_texture_width = spectrum_data.len() as u32;
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
            wave_stream,
            wave_texture_width,
            spectrum_texture_width,
            wave_texture,
            spectrum_texture,
            wave_view,
            spectrum_view,
            sampler,
        }
    }
    fn create_1d_texture(
        graphics: &std::rc::Rc<GraphicsDevice>,
        wave_data: &Vec<f32>,
        label: &str,
    ) -> wgpu::Texture {
        log::trace!("Creating audio resource texture");
        let descriptor = wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: wave_data.len() as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D1,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };
        graphics.device.create_texture(&descriptor)
    }
}

impl<const BIND_GROUP: u32, const BIND_OFFSET: u32> CubensisResource<BIND_GROUP, BIND_OFFSET, 3>
    for AudioResource<BIND_GROUP, BIND_OFFSET>
{
    fn update(&mut self, _: Duration) -> bool {
        log::trace!("Updating audio buffer resource");
        let mut should_rebuild_bind_group = false;
        let (wave_data, spectrum_data) = self.wave_stream.get_wave_and_spectrum_data();
        if wave_data.len() as u32 != self.wave_texture_width {
            self.wave_texture = AudioResource::<BIND_GROUP, BIND_OFFSET>::create_1d_texture(
                &self.graphics,
                wave_data,
                "Wave Data Texture",
            );
            self.wave_view = self
                .wave_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            should_rebuild_bind_group = true;
        }
        if spectrum_data.len() as u32 != self.spectrum_texture_width {
            self.spectrum_texture = AudioResource::<BIND_GROUP, BIND_OFFSET>::create_1d_texture(
                &self.graphics,
                spectrum_data,
                "Spectrum Data Texture",
            );
            self.spectrum_view = self
                .spectrum_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            should_rebuild_bind_group = true;
        }
        self.graphics.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.wave_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(wave_data),
            wgpu::ImageDataLayout::default(),
            wgpu::Extent3d {
                width: wave_data.len() as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        self.graphics.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.spectrum_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(spectrum_data),
            wgpu::ImageDataLayout::default(),
            wgpu::Extent3d {
                width: spectrum_data.len() as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        should_rebuild_bind_group
    }

    fn resize(&mut self) {
        log::trace!("Resizing audio buffer resource");
    }

    fn get_bind_group_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
        log::trace!("Retrieving audio buffer resource bind group layout entries");
        vec![
            wgpu::BindGroupLayoutEntry {
                binding: BIND_OFFSET,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D1,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: BIND_OFFSET + 1,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D1,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: BIND_OFFSET + 2,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                count: None,
            },
        ]
    }

    fn get_bind_group_entries(&self) -> Vec<wgpu::BindGroupEntry> {
        log::trace!("Retrieving audio buffer resource bind group entries");
        vec![
            wgpu::BindGroupEntry {
                binding: BIND_OFFSET,
                resource: wgpu::BindingResource::TextureView(&self.wave_view),
            },
            wgpu::BindGroupEntry {
                binding: BIND_OFFSET + 1,
                resource: wgpu::BindingResource::TextureView(&self.spectrum_view),
            },
            wgpu::BindGroupEntry {
                binding: BIND_OFFSET + 2,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            },
        ]
    }

    fn handle_or_capture_event(&mut self, _event: &winit::event::Event<'_, CubensisEvent>) -> bool {
        log::trace!("Handling event in audio buffer resource");
        false
    }
}
