#[derive(Clone, Debug)]
pub struct AudioStreamInfo {
    pub device_name: String,
}

impl AudioStreamInfo {
    pub fn new(device_name: String) -> Self {
        Self { device_name }
    }
}
