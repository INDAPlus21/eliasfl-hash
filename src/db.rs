use std::sync::{Arc, RwLock};

use crate::hash::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Get,
    Insert,
    Remove,
}

pub struct Database<K, V> {
    records: Arc<RwLock<HashMap<K, V>>>,
}

impl Database<u64, String> {
    pub fn new() -> Self {
        let map = HashMap::new();
        let records = Arc::new(RwLock::new(map));
        Self { records }
    }

    pub fn get(&self, key: u64) -> Option<String> {
        self.records
            .read()
            .ok()
            .and_then(|guard| guard.get(&key.into()).map(|val| val.clone()))
    }
    pub fn insert(&self, key: u64, val: String) -> Option<String> {
        self.records
            .write()
            .ok()
            .and_then(|mut guard| guard.insert(key, val).map(|val| val.clone()))
    }
    pub fn remove(&self, key: u64) -> Option<String> {
        self.records
            .write()
            .ok()
            .and_then(|mut guard| guard.remove(&key.into()).map(|val| val.clone()))
    }
}
