use std::fmt::Debug;
use std::path::{Path, PathBuf};

pub fn qualify_path<P: AsRef<Path> + Debug>(file_path: &P) -> anyhow::Result<PathBuf> {
    std::fs::canonicalize(&file_path).map_err(|e| {
        log::warn!("Could not qualify nonexistent path from '{:?}'", file_path);
        e.into()
    })
}

pub fn read_to_string<P: AsRef<Path> + Debug>(file_path: &P) -> anyhow::Result<String> {
    let path = qualify_path(&file_path)?;
    std::fs::read_to_string(path).map_err(|e| {
        log::warn!("Failed to read file from path '{:?}'", file_path);
        e.into()
    })
}
