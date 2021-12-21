use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use std::cell::RefCell;

pub struct GraphicsDevice {
    surface: RefCell<wgpu::Surface>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface_configuration: RefCell<wgpu::SurfaceConfiguration>,
    size: RefCell<winit::dpi::PhysicalSize<u32>>,
    pub window_scale_factor: f64,
    event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
}

impl GraphicsDevice {
    pub fn new(
        configuration: Configuration,
        window: &winit::window::Window,
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
    ) -> Self {
        log::debug!("Creating new GraphicsDevice");
        let size = window.inner_size();
        let backend_preference = if configuration.graphics.prefer_legacy_backends {
            wgpu::Backends::SECONDARY
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(backend_preference);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .enumerate_adapters(backend_preference)
            .filter(|adapter| surface.get_preferred_format(&adapter).is_some())
            .next()
            .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default()
                    | wgpu::Features::SHADER_FLOAT64
                    | wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::BUFFER_BINDING_ARRAY
                    | wgpu::Features::VERTEX_WRITABLE_STORAGE,
                //values initialized from defaults
                limits: wgpu::Limits {
                    max_texture_dimension_1d: 8192,
                    max_texture_dimension_2d: 8192,
                    max_texture_dimension_3d: 2048,
                    max_texture_array_layers: 256,
                    max_bind_groups: 4,
                    max_dynamic_uniform_buffers_per_pipeline_layout: 8,
                    max_dynamic_storage_buffers_per_pipeline_layout: 4,
                    max_sampled_textures_per_shader_stage: 16,
                    max_samplers_per_shader_stage: 16,
                    max_storage_buffers_per_shader_stage: 8,
                    max_storage_textures_per_shader_stage: 8,
                    max_uniform_buffers_per_shader_stage: 12,
                    max_uniform_buffer_binding_size: 16384,
                    max_storage_buffer_binding_size: 128 << 20,
                    max_vertex_buffers: 8,
                    max_vertex_attributes: 16,
                    max_vertex_buffer_array_stride: 2048,
                    max_push_constant_size: 0,
                    min_uniform_buffer_offset_alignment: 256,
                    min_storage_buffer_offset_alignment: 256,
                },
                label: None,
            },
            None,
        ))
        .unwrap();
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: if configuration.graphics.enable_vsync {
                wgpu::PresentMode::Fifo
            } else {
                wgpu::PresentMode::Mailbox
            },
        };
        surface.configure(&device, &surface_configuration);
        let window_scale_factor = window.scale_factor();
        let surface = RefCell::new(surface);
        let surface_configuration = RefCell::new(surface_configuration);
        let size = RefCell::new(size);
        Self {
            surface,
            device,
            queue,
            surface_configuration,
            size,
            event_proxy,
            window_scale_factor,
        }
    }
    pub(crate) fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        log::trace!("Retrieving presentation texture for the current frame");
        self.surface.borrow().get_current_texture()
    }
    pub fn create_extent3d(&self, extent_depth: u32) -> wgpu::Extent3d {
        log::trace!("Creating surface-configured Extent3d");
        let configuration = self.surface_configuration.borrow();
        wgpu::Extent3d {
            width: configuration.width,
            height: configuration.height,
            depth_or_array_layers: extent_depth,
        }
    }
    pub fn get_format(&self) -> wgpu::TextureFormat {
        log::trace!("Retrieve surface format");
        let configuration = self.surface_configuration.borrow();
        configuration.format
    }
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        log::trace!("Retrieving window size");
        self.size.clone().into_inner()
    }
    pub(crate) fn get_event_proxy(&self) -> winit::event_loop::EventLoopProxy<CubensisEvent> {
        log::trace!("Cloning device event proxy");
        self.event_proxy.clone()
    }
    pub(crate) fn resize(&self, size: winit::dpi::PhysicalSize<u32>) -> bool {
        log::trace!("Resizing device");
        if size.width > 0 && size.height > 0 {
            self.size.replace(size);
            let mut surface_configuration = self.surface_configuration.borrow_mut();
            surface_configuration.width = size.width;
            surface_configuration.height = size.height;
            let surface = self.surface.borrow_mut();
            surface.configure(&self.device, &surface_configuration);
            log::trace!("Resize device");
            return true;
        }
        log::trace!("Screen dimensions invalid, couldn't resize");
        false
    }
}
