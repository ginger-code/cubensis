use crate::device::GraphicsDevice;
use crate::gui::{CubensisGuiApp, CubensisGuiRenderer, GuiHost};
use crate::mesh::buffers::{MeshBuffers, Vertex};
use crate::presentation::depth_texture::DepthTexture;
use crate::resources::CubensisResourceCollection;
use hyphae::configuration::library::Library;
use hyphae::scene::geometry::GeometrySource;
use hyphae::scene::primitives::PrimitiveType::Quad;
use std::rc::Rc;
use textures::PresentTexture;

mod depth_texture;
pub mod presenter;
mod textures;

pub struct PresentationPass<const HISTORY_DEPTH: usize> {
    render_history: Vec<PresentTexture>,
    graphics: Rc<GraphicsDevice>,
    buffers: MeshBuffers,
    pipeline: wgpu::RenderPipeline,
    depth_texture: DepthTexture,
}

impl<const HISTORY_DEPTH: usize> PresentationPass<HISTORY_DEPTH> {
    pub fn new(graphics: Rc<GraphicsDevice>) -> Self {
        log::debug!("Creating presentation pass");
        let size = graphics.create_extent3d(1);
        let format = graphics.get_format();

        let render_history = Self::create_textures(&graphics.device, size, format);
        let buffers = MeshBuffers::new(&graphics.device, &GeometrySource::Primitive(Quad));
        let pipeline = Self::create_pipeline(&graphics.device, graphics.get_format());
        let depth_texture = DepthTexture::new(&graphics.device, size);

        Self {
            render_history,
            graphics,
            buffers,
            pipeline,
            depth_texture,
        }
    }

    pub fn get_current_bind_group(&self) -> &wgpu::BindGroup {
        &self.render_history[1].bind_group
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.render_history[0].bind_group_layout
    }

    pub fn get_depth_texture_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.texture_view
    }

    pub fn start_frame(&mut self) {
        self.rotate_textures();
    }

    pub fn create_presentation_view(&self) -> &wgpu::TextureView {
        log::trace!("Creating presentation pass view");
        &self.render_history[0].texture_view
    }
    pub fn resize<'b>(&mut self) {
        log::trace!("Resizing presentation pass");
        let size = self.graphics.create_extent3d(1);
        let format = self.graphics.get_format();
        self.pipeline = Self::create_pipeline(&self.graphics.device, self.graphics.get_format());
        self.render_history = Self::create_textures(&self.graphics.device, size, format);
        self.depth_texture = DepthTexture::new(&self.graphics.device, size);
    }

    fn create_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Presentation bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: false,
                    },
                    count: None,
                },
            ],
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Present Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../shaders/present_shader.wgsl").into(),
            ),
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Presentation Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        })
    }
    fn present<
        ResourceCollection: CubensisResourceCollection,
        Gui: CubensisGuiApp<ResourceCollection>,
    >(
        &mut self,
        mut encoder: wgpu::CommandEncoder,
        gui_host: &mut GuiHost,
        gui: &mut Gui,
        library: &Library,
        resource_collection: &ResourceCollection,
    ) -> Result<(), wgpu::SurfaceError> {
        log::trace!("Presenting image");
        let surface_texture = self.graphics.get_current_texture()?;
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Presentation Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.render_history[0].bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.buffers.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.buffers.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.buffers.index_count, 0, 0..1);
        }
        encoder.render_gui(gui_host, gui, library, resource_collection, &surface_view);
        self.graphics
            .queue
            .submit(std::iter::once(encoder.finish()));
        surface_texture.present();
        Ok(())
    }
    fn rotate_textures(&mut self) {
        log::trace!("Rotating history images");
        self.render_history.rotate_right(1);
    }

    fn create_textures(
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> Vec<PresentTexture> {
        log::debug!("Creating {} presentation pass textures", HISTORY_DEPTH + 1);
        let mut res = Vec::new();
        for i in 0..(HISTORY_DEPTH + 1) as u32 {
            res.push(PresentTexture::new(
                device,
                size,
                format,
                wgpu::TextureDimension::D2,
                format!("Render History Texture {:?}", i).as_str(),
            ));
        }
        log::debug!("Created {} presentation pass textures", res.len());
        res
    }
}
