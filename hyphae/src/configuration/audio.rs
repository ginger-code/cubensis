#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AudioConfiguration {
    buffer_size: usize,
}

impl AudioConfiguration {
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl Default for AudioConfiguration {
    fn default() -> Self {
        Self { buffer_size: 4096 }
    }
}
