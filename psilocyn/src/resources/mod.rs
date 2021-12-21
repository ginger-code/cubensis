use crate::device::GraphicsDevice;
use crate::mesh::Mesh;
use hyphae::configuration::library::LibraryConfiguration;
use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::scene::Scene;
use std::path::PathBuf;
use std::rc::Rc;
use wgpu::BindGroupLayout;

pub trait CubensisResource<
    const BIND_GROUP: u32,
    const BIND_OFFSET: u32,
    const BIND_ENTRY_COUNT: usize,
>
{
    ///returns true if bind group should be recreated
    fn update(&mut self, time_delta: std::time::Duration) -> bool;
    fn resize(&mut self);
    fn get_bind_group_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry>;
    fn get_bind_group_entries(&self) -> Vec<wgpu::BindGroupEntry>;
    fn handle_or_capture_event(&mut self, event: &winit::event::Event<'_, CubensisEvent>) -> bool;
}

pub trait CubensisResourceCollection {
    fn new(graphics: Rc<GraphicsDevice>, configuration: Configuration) -> Self;
    fn update(&mut self, time_delta: std::time::Duration);
    fn resize(&mut self);
    fn get_bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout>;
    fn get_bind_groups(&self) -> Vec<&wgpu::BindGroup>;
    fn handle_event(&mut self, event: &winit::event::Event<'_, CubensisEvent>);
}

pub trait ResourceCollectionBinder<'a, ResourceCollection: CubensisResourceCollection> {
    fn bind_resource_collection(&mut self, resource_collection: &'a ResourceCollection);
}

impl<'a, ResourceCollection: CubensisResourceCollection>
    ResourceCollectionBinder<'a, ResourceCollection> for wgpu::RenderPass<'a>
{
    fn bind_resource_collection(&mut self, resource_collection: &'a ResourceCollection) {
        log::trace!("Binding resource collection");
        let bind_groups = resource_collection.get_bind_groups();
        for bind_group in bind_groups {
            self.set_bind_group(0, bind_group, &[]);
        }
    }
}

pub trait CubensisMeshSpawner<ResourceCollection: CubensisResourceCollection> {
    fn create_meshes(
        &self,
        graphics: Rc<GraphicsDevice>,
        resource_collection: &ResourceCollection,
        history_bind_group_layout: &BindGroupLayout,
    ) -> Vec<Mesh>;
    fn try_hot_reload(
        &self,
        path: &PathBuf,
        meshes: &mut Vec<Mesh>,
        resource_collection: &ResourceCollection,
        history_bind_group_layout: &BindGroupLayout,
    ) -> anyhow::Result<()>;
}

impl<ResourceCollection: CubensisResourceCollection> CubensisMeshSpawner<ResourceCollection>
    for Scene
{
    fn create_meshes(
        &self,
        graphics: Rc<GraphicsDevice>,
        resource_collection: &ResourceCollection,
        history_bind_group_layout: &BindGroupLayout,
    ) -> Vec<Mesh> {
        log::debug!("Creating meshes");
        self.meshes
            .iter()
            .map(|m| {
                Mesh::new(
                    graphics.clone(),
                    m.clone(),
                    resource_collection.get_bind_group_layouts().as_slice(),
                    history_bind_group_layout,
                )
            })
            .collect()
    }

    fn try_hot_reload(
        &self,
        path: &PathBuf,
        meshes: &mut Vec<Mesh>,
        resource_collection: &ResourceCollection,
        history_bind_group_layout: &BindGroupLayout,
    ) -> anyhow::Result<()> {
        log::debug!("Attempting hot reload of shader at path {:?}", path);
        for mesh in meshes.iter_mut() {
            let shader_to_update = mesh.mesh_descriptor.render_shaders.iter().find(|s| {
                let mut ref_path = std::path::Path::new(&s.path);
                let canonical_path = LibraryConfiguration::scene_library_path()
                    .join(ref_path)
                    .canonicalize()
                    .unwrap();
                if ref_path.is_relative() {
                    ref_path = &canonical_path;
                }
                let res = path.canonicalize().unwrap() == ref_path;
                log::debug!("Checking ({}) for path {:?}", &res, &ref_path);
                res
            });
            if let Some(render_shader) = shader_to_update {
                log::debug!("Rebuilding shader named {}", render_shader.name);
                let render_shader = render_shader.clone();
                mesh.rebuild(
                    render_shader,
                    resource_collection,
                    history_bind_group_layout,
                )?;
            }

            return Ok(());
        }
        Err(anyhow::Error::msg(
            "Updated path doesn't correspond to any loaded meshes/shaders",
        ))
    }
    // let extension = path.extension().map(|e| e.to_str()).unwrap_or("".into());
    // todo: this block is potentially useful for rebuilding the scene, but not needed for shader hot-reload
    // if let Some("cubensis-scene") = extension {
    //     let mut new_meshes =
    //         self.create_meshes(graphics, resource_collection, history_bind_group_layout);
    //     meshes.clear();
    //     meshes.append(&mut new_meshes);
    // }
}
