use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ReservedPorts {
    pub(crate) entries: Arc<Mutex<BTreeSet<u32>>>,
}

impl Default for ReservedPorts {
    fn default() -> Self {
        ReservedPorts { entries : Arc::new(Mutex::new(BTreeSet::new())) }
    }
}

impl ReservedPorts {
    pub fn reserve_next(&self, from: Option<u32>) -> u32 {
        let next = self.next(from);
        let mut set = self.entries.lock().unwrap();
        set.insert(next);
        next
    }

    pub fn next(&self, from: Option<u32>) -> u32 {
        let mut next: u32 = from.unwrap_or(1);
        let set = self.entries.lock().unwrap();
        while set.contains(&next) {
            next += 1;
        }
        next
    }

    pub fn release(&self, value: u32) -> bool {
        let mut set = self.entries.lock().unwrap();
        set.remove(&value)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::BTreeSet;
    use std::sync::{Arc, Mutex};
    use crate::reserved_ports::ReservedPorts;

    #[test]
    fn next_with_default() {
        let store = ReservedPorts;
        assert_eq!(store.next(None), 1);
    }

    #[test]
    fn next_with_value_and_no_existing_values() {
        let store = ReservedPorts;
        assert_eq!(store.next(Some(1234)), 1234);
    }

    #[test]
    fn next_with_value_and_existing_values() {
        let store = ReservedPorts { entries: Arc::new(Mutex::new(BTreeSet::from([1, 2, 3, 4 /*, 5*/, 6 /*, 7*/, 8]))) };
        assert_eq!(store.next(Some(2)), 5);
        assert_eq!(store.next(Some(6)), 7);
        assert_eq!(store.next(Some(9)), 9);
    }

    #[test]
    fn reserve_next_with_default() {
        let mut store = ReservedPorts;
        assert_eq!(store.reserve_next(None), 1);
    }

    #[test]
    fn reserve_next_with_value_and_no_existing_values() {
        let mut store = ReservedPorts;
        assert_eq!(store.reserve_next(Some(1234)), 1234);
    }

    #[test]
    fn reserve_next_with_value_and_existing_values() {
        let mut store = ReservedPorts { entries: Arc::new(Mutex::new(BTreeSet::from([1, 2, 4]))) };
        assert_eq!(store.reserve_next(Some(2)), 3);
    }

    #[test]
    fn release_with_no_value() {
        let store = ReservedPorts;
        assert!(!store.release(1));
    }

    #[test]
    fn release_with_values() {
        let store = ReservedPorts { entries: Arc::new(Mutex::new(BTreeSet::from([1, 2, 4]))) };
        assert!(store.clone().release(2));
        assert!(!store.release(3));
    }
}