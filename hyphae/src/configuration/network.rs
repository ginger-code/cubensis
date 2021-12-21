#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NetworkConfiguration {
    port: u16,
}

impl NetworkConfiguration {
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }
}

impl Default for NetworkConfiguration {
    fn default() -> Self {
        Self { port: 3751 }
    }
}
