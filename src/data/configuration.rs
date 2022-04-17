use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::data::template::Template;
use crate::PortStore;

#[derive(Clone, Serialize)]
pub struct Configuration {
    pub uuid: String,
    pub content: String,
    pub ports: Vec<u32>
}

pub fn generate(template: Template, port_store: Arc<PortStore>) -> Configuration {
    let uuid = Uuid::new_v4().to_string();

    let mut content = template.content;
    let mut ports: Vec<u32> = Vec::new();
    for port_request in template.port_requests {
        let port = port_store.reserve_next(Some(port_request.requested_value));
        ports.push(port);
        let guard = port_request.matches.lock().unwrap();
        for m in guard.iter() {
            content = content.replace(m, &port.to_string());
        }
    }

    Configuration {
        uuid: uuid.clone(),
        content,
        ports
    }
}
