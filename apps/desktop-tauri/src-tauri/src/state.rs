use std::sync::{Arc, Mutex};

use storage::db::{self, DB, Session};
use sync::SuppresionCache;

pub struct AppState {
    db: Mutex<DB>,
}

pub struct CacheState {
    cache: Arc<SuppresionCache>,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub token: String,
    pub device_id: String,
}

pub struct AuthState {
    session: Mutex<Option<AuthSession>>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = db::setup()?;
        Ok(Self { db: Mutex::new(db) })
    }

    pub fn db(&self) -> &Mutex<DB> {
        &self.db
    }

    pub fn load_auth_session(&self) -> Result<Option<AuthSession>, Box<dyn std::error::Error>> {
        let db = self.db.lock().unwrap();
        let session = db.get_auth()?.map(AuthSession::from);
        Ok(session)
    }
}

impl CacheState {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(SuppresionCache::new()),
        }
    }

    pub fn cache(&self) -> Arc<SuppresionCache> {
        Arc::clone(&self.cache)
    }
}

#[allow(dead_code)]
impl AuthState {
    pub fn new(session: Option<AuthSession>) -> Self {
        Self {
            session: Mutex::new(session),
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.session.lock().unwrap().is_some()
    }

    pub fn session(&self) -> Option<AuthSession> {
        self.session.lock().unwrap().clone()
    }

    pub fn set_session(&self, session: Option<AuthSession>) {
        *self.session.lock().unwrap() = session;
    }
}

impl From<Session> for AuthSession {
    fn from(value: Session) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            email: value.email,
            token: value.token,
            device_id: value.device_id,
        }
    }
}
