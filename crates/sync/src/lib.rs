use std::fmt;
use std::{collections::HashSet, sync::Mutex};

pub struct SuppresionCache {
    inner: Mutex<HashSet<String>>,
}

impl SuppresionCache {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashSet::new()),
        }
    }

    pub fn insert(&self, value: String) -> String {
        let hash = hash(&value);
        self.inner.lock().unwrap().insert(hash.clone());
        hash
    }

    /// Returns true if the given *raw* value is suppressed.
    pub fn contains(&self, value: &str) -> bool {
        self.contains_value(value)
    }

    /// Returns true if the given *raw* value is suppressed.
    pub fn contains_value(&self, value: &str) -> bool {
        let hash = hash(value);
        self.contains_hash(&hash)
    }

    /// Returns true if the given *hash* is present in the suppression set.
    pub fn contains_hash(&self, hash: &str) -> bool {
        self.inner.lock().unwrap().contains(hash)
    }

    pub fn remove(&self, hash: &str) {
        self.inner.lock().unwrap().remove(hash);
    }
}

impl Default for SuppresionCache {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SuppresionCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.lock() {
            Ok(guard) => write!(f, "{:?}", &*guard),
            Err(_) => write!(f, "<SuppresionCache: poisoned lock>"),
        }
    }
}

pub fn hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_string()
}
