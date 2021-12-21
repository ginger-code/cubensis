#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GraphicsConfiguration {
    pub enable_vsync: bool,
    pub prefer_legacy_backends: bool,
}

impl GraphicsConfiguration {}

impl Default for GraphicsConfiguration {
    fn default() -> Self {
        Self {
            enable_vsync: false,
            prefer_legacy_backends: false,
        }
    }
}
