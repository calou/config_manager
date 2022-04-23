use std::collections::BTreeSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PortList {
    pub entries: BTreeSet<u32>,
}

impl PortList {
    pub fn next(&self, from: Option<u32>) -> u32 {
        let mut next: u32 = from.unwrap_or(1);
        while self.entries.contains(&next) {
            next += 1;
        }
        next
    }
}

impl Default for PortList {
    fn default() -> Self {
        PortList { entries: BTreeSet::new() }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use crate::data::port_list::PortList;

    #[test]
    fn next_with_value_and_existing_values() {
        let store = PortList { entries: BTreeSet::from([1, 2, 3, 4 /*, 5*/, 6 /*, 7*/, 8]) };
        assert_eq!(store.next(Some(2)), 5);
        assert_eq!(store.next(Some(6)), 7);
        assert_eq!(store.next(Some(9)), 9);
    }
}