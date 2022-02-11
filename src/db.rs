use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use crate::hash::HashMap;

#[derive(Debug, Clone)]
pub struct ArgParseError(String);
impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse argument action: {}", self.0)
    }
}
impl Error for ArgParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Get,
    Set,
    Insert,
    Remove,
}

impl FromStr for Action {
    type Err = ArgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "get" => Ok(Action::Get),
            "set" => Ok(Action::Set),
            "insert" => Ok(Action::Insert),
            "remove" => Ok(Action::Remove),
            string => Err(ArgParseError(string.to_owned())),
        }
    }
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
}
