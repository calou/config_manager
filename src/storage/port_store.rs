use std::sync::{Arc, Mutex};
use rocksdb::DB;
use crate::data::port_list::PortList;


const PORT_KEY: &[u8; 5] = b"ports";

#[derive(Clone)]
pub struct PortStore {
    pub db: Arc<Mutex<DB>>,
}

impl PortStore {
    pub fn new(db: DB) -> Self {
        if !db.key_may_exist(PORT_KEY) {
            let port_list: PortList = PortList::default();
            let byte_array = serde_json::to_vec(&port_list).unwrap();
            let _ = db.put(PORT_KEY, &byte_array);
        }
        PortStore {
            db: Arc::new(Mutex::new(db))
        }
    }

    pub fn reserve_next(&self, from: Option<u32>) -> u32 {
        let mut next: u32 = 0;
        let guard = self.db.lock().unwrap();
        if let Some(entries) = guard.get(PORT_KEY).unwrap() {
            let mut port_list: PortList = serde_json::from_slice(entries.as_slice()).unwrap();
            next = port_list.next(from);
            port_list.entries.insert(next);
            let byte_array = serde_json::to_vec(&port_list).unwrap();
            let _ = guard.put(PORT_KEY, &byte_array);
        }
        next
    }

    pub fn release(&self, value: u32) -> bool {
        let mut result: bool = false;
        let guard = self.db.lock().unwrap();
        if let Some(entries) = guard.get(PORT_KEY).unwrap() {
            let mut port_list: PortList = serde_json::from_slice(entries.as_slice()).unwrap();
            result = port_list.entries.remove(&value);
            let byte_array = serde_json::to_vec(&port_list).unwrap();
            let _ = guard.put(PORT_KEY, &byte_array);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use rocksdb::{DB, Options};
    use uuid::Uuid;
    use crate::storage::port_store::{PORT_KEY, PortList, PortStore};

    fn delete_store(store: PortStore) {
        let _ = DB::destroy(&Options::default(), store.db.lock().unwrap().path());
    }

    fn create_store(arr: Vec<u32>) -> PortStore {
        let path = format!("/tmp/test_{}.db", Uuid::new_v4().to_string());
        let db = DB::open_default(path).unwrap();
        let mut entries: BTreeSet<u32> = BTreeSet::new();
        for &v in arr.iter() {
            entries.insert(v);
        }
        let port_list = PortList { entries };
        let _ = db.put(PORT_KEY, serde_json::to_vec(&port_list).unwrap());
        let store = PortStore::new(db);
        store
    }

    #[test]
    fn reserve_next_with_default() {
        let store = create_store(vec![]);
        assert_eq!(store.reserve_next(None), 1);
        delete_store(store);
    }

    #[test]
    fn reserve_next_with_value_and_no_existing_values() {
        let store = create_store(vec![]);
        assert_eq!(store.reserve_next(Some(1234)), 1234);
        delete_store(store);
    }

    #[test]
    fn reserve_next_with_value_and_existing_values() {
        let store = create_store(vec![1, 2]);
        assert_eq!(store.reserve_next(Some(2)), 3);
        delete_store(store);
    }

    #[test]
    fn release_with_no_value() {
        let store = create_store(vec![]);
        assert!(!store.release(1));
        delete_store(store);
    }

    #[test]
    fn release_with_values() {
        let store = create_store(vec![1, 2, 4]);
        assert!(store.clone().release(2));
        assert!(!store.release(3));
        delete_store(store);
    }
}