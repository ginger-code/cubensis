#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub enum RpcRequest {
    SetProject { project_name: String },
}
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
#[repr(u32)]
pub enum RpcSeverity {
    None = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct RpcResponse {
    pub is_error: bool,
    pub severity: RpcSeverity,
    pub message: String,
}
impl RpcResponse {
    pub fn success(message: Option<String>, severity: Option<RpcSeverity>) -> Self {
        Self {
            is_error: false,
            severity: severity.unwrap_or(RpcSeverity::None),
            message: message.unwrap_or("".to_string()),
        }
    }
    pub fn error(message: String, severity: Option<RpcSeverity>) -> Self {
        Self {
            is_error: true,
            severity: severity.unwrap_or(RpcSeverity::Error),
            message,
        }
    }

    pub(crate) fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
