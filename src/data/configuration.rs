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
        let matches = port_request.matches.lock().unwrap();
        for m in matches.iter() {
            content = content.replace(m, &port.to_string());
        }
    }

    Configuration {
        uuid,
        content,
        ports
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rocksdb::{DB, Options};
    use uuid::Uuid;
    use crate::data::configuration::generate;
    use crate::data::template::Template;
    use crate::PortStore;

    fn delete_store(store: PortStore) {
        let _ = DB::destroy(&Options::default(), store.db.lock().unwrap().path());
    }

    fn create_store() -> PortStore {
        let path = format!("/tmp/test_{}.db", Uuid::new_v4().to_string());
        let db = DB::open_default(path).unwrap();
        let store = PortStore::new(db);
        store
    }

    #[test]
    fn generate_returns_expected_data() {
        let template = Template::create("test:\n\t%abc:1234%\n\tTEST: %def%\n");
        let port_store = create_store();

        let configuration = generate(template, Arc::new(port_store.clone()));
        assert_eq!(configuration.content, "test:\n\t1234\n\tTEST: 3001\n");
        delete_store(port_store);
    }
}