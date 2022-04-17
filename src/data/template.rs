use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Template {
    pub uuid: String,
    pub content: String,
    pub port_requests: Vec<PortRequest>,
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

lazy_static! {
    static ref RE: Regex = Regex::new(r"%(\w+)(:(\d+))?%").unwrap();
}

pub fn parse(content: String) -> Vec<PortRequest> {
    let mut port_requests_by_name: HashMap<String, PortRequest> = HashMap::new();

    for cap in RE.captures_iter(content.as_str()) {
        let name = cap.get(1).map_or("", |m| m.as_str());
        let port_as_atring = cap.get(3).map_or("3001", |m| m.as_str());
        let port = port_as_atring.parse().unwrap();
        if let Some(port_request) = port_requests_by_name.get(name) {
            port_request.add_match(cap[0].to_string());
        } else {
            port_requests_by_name.insert(name.clone().to_string(), PortRequest {
                name: name.to_string(),
                requested_value: port,
                matches: Arc::new(Mutex::new(HashSet::new())),
            });
        }
    }

    return port_requests_by_name.clone()
        .values()
        .cloned()
        .collect::<Vec<PortRequest>>();
}


#[cfg(test)]
mod tests {
    use regex::escape;
    use crate::data::template::parse;

    #[test]
    fn parse_returns_expected_data() {
        println!("{}", escape(":"));
        let port_requests = parse(String::from("test %abc:3000%, TEST: %def%"));
        assert_eq!(port_requests.get(0).unwrap().name, "abc".to_string());
        assert_eq!(port_requests.get(1).unwrap().name, "def".to_string());
    }
}