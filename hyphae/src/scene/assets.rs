#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TextureAsset {
    pub path: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct SceneTextures {
    pub texture_1: Option<TextureAsset>,
    pub texture_2: Option<TextureAsset>,
    pub texture_3: Option<TextureAsset>,
    pub texture_4: Option<TextureAsset>,
    pub texture_5: Option<TextureAsset>,
}
