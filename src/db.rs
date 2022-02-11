use std::fs;
use std::path::PathBuf;
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
    file: PathBuf,
}

impl Database<u64, String> {
    pub fn new<S: AsRef<str>>(filename: S) -> Self {
        let path = PathBuf::from(filename.as_ref());
        let map = if path.exists() {
            HashMap::new()
        } else {
            HashMap::new()
        };
        let records = Arc::new(RwLock::new(map));
        Self {
            records,
            file: path,
        }
    }

    pub fn get(&self, key: u64) -> Option<String> {
        self.records
            .read()
            .ok()
            .and_then(|guard| guard.get(&key.into()).map(|val| val.clone()))
    }
    pub fn insert(&self, key: u64, val: String) -> Option<String> {
        let records = self.records.write();
        let mut guard = records.unwrap();
        let result = guard.insert(key, val).map(|val| val.clone());
        serde_json::to_writer(
            fs::File::create(&self.file).unwrap(),
            &self.records.read().unwrap().clone(),
        )
        .unwrap();
        result
    }
    pub fn remove(&self, key: u64) -> Option<String> {
        let records = self.records.write();
        records
            .ok()
            .and_then(|mut guard| guard.remove(&key.into()).map(|val| val.clone()))
    }
}
