use std::collections::BTreeSet;

#[derive(Clone)]
pub struct ReservedPorts {
    pub(crate) entries: BTreeSet<u32>,
}

impl Default for ReservedPorts {
    fn default() -> Self {
        ReservedPorts { entries : BTreeSet::new() }
    }
}

impl ReservedPorts {
    pub fn reserve_next(&mut self, from: Option<u32>) -> u32 {
        let next = self.next(from);
        self.entries.insert(next);
        next
    }

    pub fn next(&self, from: Option<u32>) -> u32 {
        let mut next: u32 = from.unwrap_or(1);

        while self.entries.contains(&next) {
            next += 1;
        }
        next
    }

    pub fn release(mut self, value: u32) -> bool {
        self.entries.remove(&value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use crate::reserved_ports::ReservedPorts;

    #[test]
    fn next_with_default() {
        let store = ReservedPorts { entries: BTreeSet::new() };
        assert_eq!(store.next(None), 1);
    }

    #[test]
    fn next_with_value_and_no_existing_values() {
        let store = ReservedPorts { entries: BTreeSet::new() };
        assert_eq!(store.next(Some(1234)), 1234);
    }

    #[test]
    fn next_with_value_and_existing_values() {
        let store = ReservedPorts { entries: BTreeSet::from([1, 2, 3, 4 /*, 5*/, 6 /*, 7*/, 8]) };
        assert_eq!(store.next(Some(2)), 5);
        assert_eq!(store.next(Some(6)), 7);
        assert_eq!(store.next(Some(9)), 9);
    }

    #[test]
    fn reserve_next_with_default() {
        let mut store = ReservedPorts { entries: BTreeSet::new() };
        assert_eq!(store.reserve_next(None), 1);
    }

    #[test]
    fn reserve_next_with_value_and_no_existing_values() {
        let mut store = ReservedPorts { entries: BTreeSet::new() };
        assert_eq!(store.reserve_next(Some(1234)), 1234);
    }

    #[test]
    fn reserve_next_with_value_and_existing_values() {
        let mut store = ReservedPorts { entries: BTreeSet::from([1, 2, 4]) };
        assert_eq!(store.reserve_next(Some(2)), 3);
    }

    #[test]
    fn release_with_no_value() {
        let store = ReservedPorts {entries : BTreeSet::new()};
        assert!(!store.release(1));
    }

    #[test]
    fn release_with_values() {
        let store = ReservedPorts { entries: BTreeSet::from([1, 2, 4]) };
        assert!(store.clone().release(2));
        assert!(!store.release(3));
    }
}