use crate::data::configuration::{generate, Configuration};
use crate::data::template::Template;
use crate::PortStore;
use rocksdb::DB;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ConfigurationStore {
    db: Arc<Mutex<DB>>,
}

impl ConfigurationStore {
    pub fn new(db: DB) -> Self {
        ConfigurationStore {
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub fn create(&self, template: Template, port_store: Arc<PortStore>) -> Configuration {
        let config = generate(template, port_store);
        let db = self.db.lock().unwrap();
        let bytes = serde_json::to_vec(&config).unwrap();
        let _ = db.put(config.uuid.as_bytes(), bytes);
        config
    }

    pub fn get(&self, uuid: String) -> Option<Configuration> {
        let guard = self.db.lock().unwrap();
        if let Some(config) = guard.get(uuid.as_bytes()).unwrap() {
            let configuration: Configuration = serde_json::from_slice(config.as_slice()).unwrap();
            Some(configuration)
        } else {
            None
        }
    }

    pub fn delete(&self, uuid: String) -> Option<Configuration> {
        let guard = self.db.lock().unwrap();
        let uuid = uuid.as_bytes();
        if let Some(config) = guard.get(uuid).unwrap() {
            let configuration: Configuration = serde_json::from_slice(config.as_slice()).unwrap();
            let _ = guard.delete(uuid);
            Some(configuration)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {}
