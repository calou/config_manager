use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use crate::data::configuration::{Configuration, generate};
use crate::data::template::Template;
use crate::PortStore;

#[derive(Clone)]
pub struct ConfigurationStore {
    map: Arc<Mutex<BTreeMap<String, Configuration>>>,
}


impl Default for ConfigurationStore {
    fn default() -> Self {
        ConfigurationStore { map: Arc::new(Mutex::new(BTreeMap::new())) }
    }
}


impl ConfigurationStore {
    pub fn create(&self, template: Template, port_store:Arc<PortStore>) -> Configuration {
        let config = generate(template, port_store);
        let mut m = self.map.lock().unwrap();
        m.insert(config.clone().uuid, config.clone());
        config
    }

    pub fn get(&self, uuid: String) -> Option<Configuration> {
        let m = self.map.lock().unwrap();
        m.get(&uuid).cloned()
    }

    pub fn delete(&self, uuid: String) -> Option<Configuration> {
        let mut m = self.map.lock().unwrap();
        let option = m.get(&uuid.clone()).cloned();
        m.remove(&uuid);
        option
    }
}

#[cfg(test)]
mod tests {


}