pub mod buffers;

use crate::device::GraphicsDevice;
use crate::mesh::buffers::{MeshBuffers, Vertex};
use crate::resources::CubensisResourceCollection;
use crate::validation::CubensisValidatedShader;
use hyphae::scene::geometry::MeshDescriptor;
use hyphae::scene::shaders::{RenderShader, RenderShaderBlending};
use std::rc::Rc;
use wgpu::{BindGroup, BindGroupLayout, BlendState};

pub struct Mesh {
    graphics: Rc<GraphicsDevice>,
    buffers: MeshBuffers,
    compute_pipeline: Option<wgpu::ComputePipeline>,
    pub render_pipelines: Vec<wgpu::RenderPipeline>,
    pub mesh_descriptor: MeshDescriptor,
}

impl Mesh {
    pub fn new(
        graphics: Rc<GraphicsDevice>,
        mesh_descriptor: MeshDescriptor,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        history_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        log::debug!("Creating mesh");
        let buffers = MeshBuffers::new(&graphics.device, &mesh_descriptor.geometry_source);
        let compute_pipeline = None; //todo: revisit compute shader implementation
        let render_pipelines = Self::create_render_pipelines(
            &graphics,
            &mesh_descriptor,
            bind_group_layouts,
            history_bind_group_layout,
        );
        Self {
            graphics,
            buffers,
            compute_pipeline,
            render_pipelines,
            mesh_descriptor: mesh_descriptor.clone(),
        }
    }

    pub fn resize<'b>(
        &mut self,
        bind_group_layouts: &'b [&wgpu::BindGroupLayout],
        history_bind_group_layout: &BindGroupLayout,
    ) {
        log::trace!("Resizing mesh");
        self.compute_pipeline = None; //todo: revisit compute shader implementation
        self.render_pipelines = Self::create_render_pipelines(
            &self.graphics,
            &self.mesh_descriptor,
            bind_group_layouts,
            history_bind_group_layout,
        );
    }

    pub fn rebuild<'b, ResourceCollection: CubensisResourceCollection>(
        &mut self,
        updated_shader: RenderShader,
        resource_collection: &ResourceCollection,
        history_bind_group_layout: &BindGroupLayout,
    ) -> anyhow::Result<()> {
        let index_to_update = self
            .mesh_descriptor
            .render_shaders
            .binary_search_by_key(&updated_shader.path, |s| s.path.clone())
            .ok();
        if index_to_update.is_none() {
            log::debug!("Failed to rebuild shader from path {}", updated_shader.path);
            return Err(anyhow::Error::msg(format!(
                "Failed to rebuild shader from path {}",
                updated_shader.path
            )));
        }
        let index_to_update = index_to_update.unwrap();
        self.mesh_descriptor.render_shaders[index_to_update] = updated_shader.clone();
        let bind_group_layouts = resource_collection.get_bind_group_layouts();
        match Self::create_render_pipeline(
            &self.graphics,
            &updated_shader,
            bind_group_layouts.as_slice(),
            history_bind_group_layout,
        ) {
            Ok(new_pipeline) => {
                self.render_pipelines[index_to_update] = new_pipeline;
                log::debug!("Reloaded shader from path {}", updated_shader.path);
                Ok(())
            }
            Err(e) => {
                log::warn!("Failed to reload shader");
                Err(e)
            }
        }
    }
    fn create_render_pipeline(
        graphics: &Rc<GraphicsDevice>,
        render_shader: &RenderShader,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        history_bind_group_layout: &BindGroupLayout,
    ) -> anyhow::Result<wgpu::RenderPipeline> {
        let validated_source = render_shader.get_shader_source().validated()?;

        log::trace!("Creating render pipeline");
        let render_pipeline_layout =
            graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: [bind_group_layouts, &[history_bind_group_layout]]
                        .concat()
                        .as_slice(),
                    push_constant_ranges: &[],
                });
        let module = graphics
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Render Shader Module"),
                source: wgpu::ShaderSource::Wgsl(validated_source.into()),
            });
        Ok(graphics
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
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
                        format: graphics.get_format(),
                        blend: render_shader.get_blend_mode().into_blend_state_option(),
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: crate::DEPTH_BUFFER_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            }))
    }

    fn create_render_pipelines(
        graphics: &Rc<GraphicsDevice>,
        mesh_descriptor: &MeshDescriptor,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        history_bind_group_layout: &BindGroupLayout,
    ) -> Vec<wgpu::RenderPipeline> {
        log::trace!("Creating render pipelines for mesh");
        mesh_descriptor
            .render_shaders
            .iter()
            .map(|s| {
                Self::create_render_pipeline(
                    graphics,
                    s,
                    bind_group_layouts,
                    history_bind_group_layout,
                )
                .unwrap() //todo: address this
            })
            .collect()
    }
}

pub trait CubensisMeshRenderPass<'a> {
    fn draw_mesh_indexed(
        &mut self,
        mesh: &'a Mesh,
        bind_groups: &'a [&wgpu::BindGroup],
        history_bind_group: &'a BindGroup,
    );

    fn draw_mesh_indirect(
        &mut self,
        mesh: &'a Mesh,
        bind_groups: &'a [&wgpu::BindGroup],
        history_bind_group: &'a BindGroup,
    );
}

impl<'a> CubensisMeshRenderPass<'a> for wgpu::RenderPass<'a> {
    fn draw_mesh_indexed(
        &mut self,
        mesh: &'a Mesh,
        bind_groups: &'a [&wgpu::BindGroup],
        history_bind_group: &'a BindGroup,
    ) {
        log::trace!("Drawing indexed mesh");

        for render_pipeline in &mesh.render_pipelines {
            self.set_pipeline(render_pipeline);
            let mut bind_group_index = 0;
            for bind_group in bind_groups {
                self.set_bind_group(bind_group_index as u32, *bind_group, &[]);
                bind_group_index += 1;
            }
            self.set_bind_group(bind_group_index, history_bind_group, &[]);
            self.set_vertex_buffer(0, mesh.buffers.vertex_buffer.slice(..));
            self.set_index_buffer(
                mesh.buffers.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            self.draw_indexed(0..mesh.buffers.index_count, 0, 0..1);
        }
    }

    fn draw_mesh_indirect(
        &mut self,
        mesh: &'a Mesh,
        bind_groups: &'a [&wgpu::BindGroup],
        history_bind_group: &'a BindGroup,
    ) {
        log::trace!("Drawing indirect mesh");
        for render_pipeline in &mesh.render_pipelines {
            self.set_pipeline(render_pipeline);
            let mut bind_group_index = 0;
            for bind_group in bind_groups {
                self.set_bind_group(bind_group_index as u32, *bind_group, &[]);
                bind_group_index += 1;
            }
            self.set_bind_group(bind_group_index, history_bind_group, &[]);
            self.set_vertex_buffer(0, mesh.buffers.vertex_buffer.slice(..));
            self.set_index_buffer(
                mesh.buffers.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            self.draw_indexed_indirect(&mesh.buffers.indirect_buffer, 0);
        }
    }
}

trait IntoBlendStateOption {
    fn into_blend_state_option(self) -> Option<BlendState>;
}

impl IntoBlendStateOption for RenderShaderBlending {
    fn into_blend_state_option(self) -> Option<BlendState> {
        match &self {
            RenderShaderBlending::None => None,
            RenderShaderBlending::Replace => Some(wgpu::BlendState::REPLACE),
            RenderShaderBlending::AlphaBlending => Some(wgpu::BlendState::ALPHA_BLENDING),
        }
    }
}
