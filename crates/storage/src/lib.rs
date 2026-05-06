use crate::db::DB;

pub mod db;
mod paths;

pub struct Storage {
    pub db: DB,
}

impl Storage {
    pub fn new() -> Self {
        let db = db::setup().expect("failed to setup db");
        Self { db }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
