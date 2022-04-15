use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::data::template::Template;

#[derive(Clone)]
pub struct TemplateStore {
    map: Arc<Mutex<BTreeMap<Uuid, Template>>>,
}


impl Default for TemplateStore {
    fn default() -> Self {
        TemplateStore { map: Arc::new(Mutex::new(BTreeMap::new())) }
    }
}


impl TemplateStore {
    pub fn create(&self, content: &str) -> Template {
        let uuid = Uuid::new_v4();

        // Parse content to extract ports
        // TODO
        let template = Template {
            uuid,
            content: String::from(content),
            ports: vec![],
        };

        let mut m = self.map.lock().unwrap();
        m.insert(uuid, template.clone());
        template
    }

    pub fn get(&self, uuid: &Uuid) -> Option<Template> {
        let m = self.map.lock().unwrap();
        m.get(uuid).cloned()
    }

    pub fn get_all(&self) -> Vec<Template> {
        let m = self.map.lock().unwrap();
        m.values().cloned().collect::<Vec<Template>>()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    use crate::data::template::Template;
    use crate::storage::template::TemplateStore;

    fn create_template(content: &str) -> Template {
        Template {
            uuid: Uuid::new_v4(),
            content: String::from(content),
            ports: vec![],
        }
    }

    #[test]
    fn create_returns_expected_data() {
        let store = TemplateStore::default();
        let content = "test";
        let template = store.create(content);
        assert_eq!(template.content, String::from(content));
    }

    #[test]
    fn get_returns_expected_data() {
        let uuid = Uuid::new_v4();
        let content = "something";
        let map = BTreeMap::from( [(uuid, create_template(content))]);
        let store = TemplateStore { map: Arc::new(Mutex::new(map))};
        let template = store.get(&uuid);
        assert_eq!(template.unwrap().content, String::from(content));
    }

    #[test]
    fn get_all_returns_expected_data() {
        let content1 = "something";
        let content2 = "something else";
        let content3 = "anything";
        let map = BTreeMap::from( [
            (Uuid::new_v4(), create_template(content1)),
            (Uuid::new_v4(), create_template(content2)),
            (Uuid::new_v4(), create_template(content3)),
        ]);
        let store = TemplateStore { map: Arc::new(Mutex::new(map))};
        let templates = store.get_all();
        assert_eq!(templates.get(0).unwrap().content, String::from(content1));
        assert_eq!(templates.get(1).unwrap().content, String::from(content2));
        assert_eq!(templates.get(2).unwrap().content, String::from(content3));
    }
}