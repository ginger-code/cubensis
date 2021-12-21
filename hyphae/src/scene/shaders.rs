use crate::configuration::library::LibraryConfiguration;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub enum RenderShaderBlending {
    None,
    Replace,
    AlphaBlending,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RenderShader {
    pub path: String,
    pub name: String,
    pub blending: RenderShaderBlending,
}

impl Default for RenderShader {
    fn default() -> Self {
        RenderShader {
            path: "../shaders/default_shader.wgsl".into(),
            name: "Default Render Shader".to_string(),
            blending: RenderShaderBlending::Replace,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ComputeShader {
    pub(crate) path: String,
    pub(crate) name: String,
}

impl RenderShader {
    pub fn get_shader_source(&self) -> String {
        std::fs::read_to_string(LibraryConfiguration::scene_library_path().join(&self.path))
            .unwrap()
    }
    pub fn get_blend_mode(&self) -> RenderShaderBlending {
        self.blending
    }

    pub fn from_json(json: String) -> anyhow::Result<Self> {
        let render_shader = serde_json::from_str(&json)?;
        Ok(render_shader)
    }
    pub fn default_second_pass() -> Self {
        RenderShader {
            path: "../shaders/default_shader_second_pass.wgsl".into(),
            name: "Default Render Shader 2nd pass".to_string(),
            blending: RenderShaderBlending::AlphaBlending,
        }
    }
}

impl ComputeShader {
    pub fn get_shader_source(&self) -> String {
        std::fs::read_to_string(LibraryConfiguration::scene_library_path().join(&self.path))
            .unwrap()
    }
}
