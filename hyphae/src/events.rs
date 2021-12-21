#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub enum CubensisEvent {
    GuiRedrawRequest {},
    ///Contains the name of the scene to change to
    SceneChange(String),
    ///Contains the path of the file that has changed
    FileEdit(std::path::PathBuf),
}
