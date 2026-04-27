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

    pub fn contains(&self, value: &str) -> bool {
        let hash = hash(value);
        self.inner.lock().unwrap().contains(&hash)
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

pub fn hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_string()
}
