use anyhow::Error;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use wgpu::BlendState;

pub mod file_util;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct VertexStage {
    pub entry_point: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FragmentStage {
    pub entry_point: String,
    pub blending_mode: BlendState,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MeshShaderMetadata {
    pub name: String,
    pub path: PathBuf,
    pub imports: Vec<PathBuf>,
    pub textures: Vec<PathBuf>,
    pub vertex_stage: VertexStage,
    pub fragment_stages: Vec<FragmentStage>,
}

#[derive(Clone, Debug)]
pub struct MeshShaderFile {
    pub header: MeshShaderMetadata,
    pub source_text: String,
}

impl MeshShaderFile {
    pub fn load<P: AsRef<Path> + Debug>(file_path: P) -> anyhow::Result<Self> {
        log::debug!("Loading mesh shader file");
        let raw_contents = file_util::read_to_string(&file_path)?;
        Self::parse(&raw_contents)
    }

    pub fn parse(source: &str) -> anyhow::Result<Self> {
        const HEADER_SEPARATOR: &str = "#END CUBENSIS";
        let split: Vec<&str> = source.split(HEADER_SEPARATOR).collect();
        if split.len() < 2 {
            return Err(Error::msg(format!("Invalid mesh shader file specified. Header and source must be separated with a line '{}'", HEADER_SEPARATOR)));
        }
        if split.len() > 2 {
            return Err(Error::msg(format!("Invalid mesh shader file specified. Only one header may be specified, which must come first. Found multiple lines marked '{}'", HEADER_SEPARATOR)));
        }
        let header_source = split[0];
        let source_text = split[1].into();
        let header = serde_json::from_str::<MeshShaderMetadata>(header_source)?;
        Ok(Self {
            header,
            source_text,
        })
    }

    pub fn compile(&self) -> anyhow::Result<String> {
        let import_stubs = (&self.header.imports)
            .into_iter()
            .map(|i| file_util::read_to_string(&i))
            .collect::<Result<Vec<String>, _>>()?;
        let mut output = "".to_string();
        for import_stub in &import_stubs {
            output.push_str(import_stub);
            #[cfg(windows)]
            output.push_str(&*"\r\n");
            #[cfg(not(windows))]
            output.push_str(&*"\n");
        }
        output.push_str(&self.source_text);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
