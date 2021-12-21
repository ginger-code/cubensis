use crate::scene::primitives::PrimitiveType;
use crate::scene::shaders::{ComputeShader, RenderShader};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MeshDescriptor {
    pub name: String,
    pub geometry_source: GeometrySource,
    pub render_shaders: Vec<RenderShader>,
}

impl Default for MeshDescriptor {
    fn default() -> Self {
        MeshDescriptor {
            name: "Default Mesh".to_string(),
            geometry_source: GeometrySource::Primitive(PrimitiveType::Quad),
            render_shaders: vec![RenderShader::default(), RenderShader::default_second_pass()],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GeometrySource {
    Primitive(PrimitiveType),
    ComputeShader(ComputeShader),
}
