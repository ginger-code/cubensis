pub mod geometry;
pub mod primitives;
pub mod shaders;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub meshes: Vec<crate::scene::geometry::MeshDescriptor>,
}

impl Scene {
    pub fn load_from_disk(path: &std::path::PathBuf) -> anyhow::Result<Self> {
        log::debug!("Loading scene from disk");
        let data = std::fs::read_to_string(path)?;
        serde_json::from_str(data.as_str()).map_err(|e| e.into())
    }
}

impl Default for Scene {
    fn default() -> Self {
        log::debug!("Retrieving scene scene");
        Self {
            name: "Default Scene".to_string(),
            meshes: vec![crate::scene::geometry::MeshDescriptor::default()],
        }
    }
}
