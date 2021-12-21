use crate::scene::Scene;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LibraryConfiguration {
    default_scene_name: String,
}

impl LibraryConfiguration {
    pub fn scene_library_path() -> std::path::PathBuf {
        super::Configuration::config_directory().join("scenes")
    }
    pub fn create_if_missing() -> anyhow::Result<()> {
        let path = Self::scene_library_path();
        if !path.exists() {
            std::fs::create_dir_all(path.clone())?;
            let scene_str = serde_json::to_string(&Scene::default())?;
            let default_scene_path = path.join("default_scene.cubensis-scene");
            std::fs::write(default_scene_path, scene_str)?;
        }
        Ok(())
    }
    pub fn build_library(&self) -> Library {
        Library::new(self.default_scene_name.clone())
    }

    fn load_scenes() -> HashMap<String, Vec<Scene>> {
        let library_directory = Self::scene_library_path();
        walkdir::WalkDir::new(library_directory)
            .follow_links(true)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                log::debug!("Inspecting {}", entry.path().to_str().unwrap());
                entry.file_type().is_file()
                    && Some(Some("cubensis-scene"))
                        == entry.path().extension().map(|ex| ex.to_str())
            })
            .filter_map(|entry| Scene::load_from_disk(&entry.into_path()).ok())
            .map(|scene| (scene.name.clone(), scene))
            .into_group_map()
    }
}

impl Default for LibraryConfiguration {
    fn default() -> Self {
        Self {
            default_scene_name: "Default Scene".to_string(),
        }
    }
}

pub struct Library {
    current_scene_name: String,
    scenes: HashMap<String, Vec<Scene>>,
}

impl Library {
    fn new(current_scene_name: String) -> Self {
        let scenes = LibraryConfiguration::load_scenes();
        log::debug!("Loading library: {:?}", scenes);
        for scene in &scenes {
            log::debug!("Importing scene with name: {}", scene.0);
        }
        let current_scene_name = match scenes.contains_key(&*current_scene_name) {
            true => current_scene_name,
            false => scenes.keys().next().unwrap().clone(),
        };
        Self {
            current_scene_name,
            scenes,
        }
    }
    pub fn current_scene(&self) -> crate::scene::Scene {
        log::debug!("Retrieving current scene from configuration");
        self.scenes[&self.current_scene_name][0].clone()
    }
}
