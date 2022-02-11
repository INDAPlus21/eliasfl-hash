use std::error::Error;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockWriteGuard};

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

impl<Key, Value> Database<Key, Value>
where
    Key: serde::de::DeserializeOwned + serde::Serialize + Hash + Eq,
    Value: serde::de::DeserializeOwned + serde::Serialize + Clone,
{
    /// Create new database
    pub fn new<S: AsRef<Path>>(filename: S) -> Self {
        let path = PathBuf::from(filename.as_ref());
        let map = if path.exists() {
            let json = fs::read_to_string(&path).expect("invalid json");
            serde_json::from_str(&json).unwrap()
        } else {
            HashMap::new()
        };
        let records = Arc::new(RwLock::new(map));
        Self {
            records,
            file: path,
        }
    }

    /// Get a value from hashmap by key
    pub fn get(&self, key: Key) -> Option<Value> {
        let guard = self.records.read().unwrap();
        guard.get(&key.into()).map(|val| val.clone())
    }
    /// Insert or replace in hashmap with key and value
    pub fn insert(&self, key: Key, val: Value) -> Option<Value> {
        let mut guard = self.records.write().unwrap();
        let result = guard.insert(key, val).map(|val| val.clone());
        self.save(&guard).expect("unable to serialize json");
        result
    }
    /// Remove entry from hashmap with key
    pub fn remove(&self, key: Key) -> Option<Value> {
        let mut guard = self.records.write().unwrap();
        let result = guard.remove(&key).map(|val| val.clone());
        self.save(&guard).expect("unable to serialize json");
        result
    }

    /// Serialize hashmap and save to database file
    pub fn save(
        &self,
        guard: &RwLockWriteGuard<HashMap<Key, Value>>,
    ) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string::<HashMap<_, _>>(&guard)?;
        fs::write(&self.file, json)?;
        Ok(())
    }
}
