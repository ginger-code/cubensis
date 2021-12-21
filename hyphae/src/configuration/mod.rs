use crate::configuration::audio::AudioConfiguration;
use crate::configuration::graphics::GraphicsConfiguration;
use crate::configuration::library::LibraryConfiguration;
use crate::configuration::network::NetworkConfiguration;
pub mod audio;
pub mod graphics;
pub mod library;
pub mod network;

pub trait CubensisConfigurationSource {
    fn get_configuration(&self) -> Configuration;
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub audio: AudioConfiguration,
    pub graphics: GraphicsConfiguration,
    pub network: NetworkConfiguration,
    pub library: LibraryConfiguration,
}

impl Default for Configuration {
    fn default() -> Self {
        log::debug!("Creating default configuration");
        Self {
            audio: AudioConfiguration::default(),
            graphics: GraphicsConfiguration::default(),
            network: NetworkConfiguration::default(),
            library: LibraryConfiguration::default(),
        }
    }
}

impl Configuration {
    pub fn config_directory() -> std::path::PathBuf {
        dirs::home_dir()
            .unwrap_or(dirs::config_dir().unwrap())
            .join(".cubensis")
    }
    pub fn config_path() -> std::path::PathBuf {
        log::trace!("Retrieving configuration path");
        Self::config_directory().join("config.json")
    }

    pub fn create_if_missing() -> anyhow::Result<()> {
        let path = Self::config_directory();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        let path = Self::config_path();
        if !path.exists() {
            log::info!(
                "Creating missing configuration file at {}",
                path.to_str().unwrap()
            );
            Self::default().save()?;
        } else {
            log::info!("Configuration file found at {}", path.to_str().unwrap());
        }
        LibraryConfiguration::create_if_missing()?;
        Ok(())
    }

    pub fn load() -> Self {
        log::trace!("Loading configuration");
        match std::fs::read_to_string(Self::config_path()) {
            Ok(config) => serde_json::from_str(config.as_str()).unwrap_or_default(),
            Err(_) => {
                log::warn!("Failed to load configuration file, falling back to default values");
                Self::default()
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        log::trace!("Saving configuration");
        std::fs::write(Self::config_path(), serde_json::to_string(&self)?).map_err(|e| e.into())
    }
}
