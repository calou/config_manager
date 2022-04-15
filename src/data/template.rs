use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Template {
    pub uuid: String,
    pub content: String,
    pub ports: Vec<PortRequest>
}

#[derive(Debug, Serialize, Clone)]
pub struct PortRequest {
    name: String,
    requested_value: u32
}
