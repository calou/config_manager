use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

lazy_static! {
    static ref RE: Regex = Regex::new(r"%(\w+)(:(\d+))?%").unwrap();
}

#[derive(Serialize, Clone)]
pub struct Template {
    pub content: String,
    pub port_requests: Vec<PortRequest>,
}

impl Template {
    pub fn create(content: &str) -> Self {
        let mut port_requests_by_name: BTreeMap<String, PortRequest> = BTreeMap::new();

        for cap in RE.captures_iter(content) {
            let name = cap.get(1).map_or("", |m| m.as_str());
            let port_as_atring = cap.get(3).map_or("3001", |m| m.as_str());
            let port = port_as_atring.parse().unwrap();
            if let Some(port_request) = port_requests_by_name.get(name) {
                port_request.add_match(cap[0].to_string());
            } else {
                let mut set = HashSet::new();
                set.insert(cap[0].to_string());
                port_requests_by_name.insert(name.to_string(), PortRequest {
                    name: name.to_string(),
                    requested_value: port,
                    matches: Arc::new(Mutex::new(set)),
                });
            }
        }


        Template {
            content: String::from(content),
            port_requests: port_requests_by_name.clone()
                .values()
                .cloned()
                .collect::<Vec<PortRequest>>()
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PortRequest {
    pub name: String,
    pub requested_value: u32,
    #[serde(skip_serializing)]
    pub matches: Arc<Mutex<HashSet<String>>>,
}

impl PortRequest {
    fn add_match(&self, m: String) {
        let mut set = self.matches.lock().unwrap();
        set.insert(m);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::template::Template;

    #[test]
    fn create_returns_expected_data() {
        let port_requests = Template::create("test %abc:1234%, TEST: %def%").port_requests;
        let first_port_requests = port_requests.get(0).unwrap();
        assert_eq!(first_port_requests.name, "abc".to_string());
        let guard = first_port_requests.matches.lock().unwrap();
        assert!(guard.contains("%abc:1234%"));
        assert_eq!(port_requests.get(1).unwrap().name, "def".to_string());
    }
}