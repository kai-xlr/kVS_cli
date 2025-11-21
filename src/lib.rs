//! A simple in-memory key-value store.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub struct KvStore {
    map: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvError {
    KeyNotFound(String),
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::KeyNotFound(key) => write!(f, "Key not found: {}", key),
        }
    }
}

impl Error for KvError {}

pub type KvResult<T> = Result<T, KvError>;

impl KvStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.map.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|v| v.as_str())
    }

    pub fn delete(&mut self, key: &str) -> KvResult<()> {
        self.map
            .remove(key)
            .map(|_| ())
            .ok_or_else(|| KvError::KeyNotFound(key.to_string()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_and_gets() {
        let mut store = KvStore::new();
        store.set("foo", "bar");
        assert_eq!(store.get("foo"), Some("bar"));
        assert_eq!(store.get("missing"), None);
    }

    #[test]
    fn it_deletes() {
        let mut store = KvStore::new();
        store.set("foo", "bar");

        assert!(store.delete("foo").is_ok());
        assert_eq!(store.get("foo"), None);

        assert!(store.delete("foo").is_err());
    }
}
