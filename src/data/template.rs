use uuid::Uuid;

#[derive(Clone)]
pub struct Template {
    pub uuid: Uuid,
    pub content: String,
    pub ports: Vec<PortRequest>
}

#[derive(Clone)]
pub struct PortRequest {
    name: String,
    requested_value: u32
}
