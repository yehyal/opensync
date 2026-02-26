use std::collections::HashMap;

use anyhow::Ok;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

use crate::model::{User, UserStatus};

pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

pub struct UpdateUserDTO {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub last_login: Option<DateTime<Utc>>,
}

pub struct InMemoryUserStore {
    pub counter: i64,
    pub items: HashMap<i64, User>,
}

pub struct UserService {
    pub data: Mutex<InMemoryUserStore>,
}

impl Default for UserService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryUserStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait UserServiceImpl {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;
    async fn create_user(&self, req: CreateUserDTO) -> anyhow::Result<User>;
    async fn update_user(&self, id: i64, req: UpdateUserDTO) -> anyhow::Result<User>;
    async fn delete_user(&self, id: i64) -> anyhow::Result<()>;
}

impl UserServiceImpl for UserService {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        let data = self.data.lock().await;

        Ok(data.items.values().map(|user| user.clone()).collect())
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let data = self.data.lock().await;

        match data.items.get(&id) {
            None => {
                anyhow::bail!("User not found: {}", id)
            }

            Some(user) => Ok((*user).clone()),
        }
    }

    async fn create_user(&self, req: CreateUserDTO) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        data.counter += 1;

        let timestamp = chrono::offset::Utc::now();

        let user = User {
            id: data.counter,
            username: req.username,
            password: req.password,
            status: req.status,
            created_at: timestamp,
            updated_at: timestamp,
            last_login: Option::None,
        };

        let user_id = user.id;
        data.items.insert(user.id, user);

        match data.items.get(&user_id) {
            None => {
                anyhow::bail!("User could not be created")
            }
            Some(user) => Ok((*user).clone()),
        }
    }

    async fn update_user(&self, id: i64, req: UpdateUserDTO) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;

        let user = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::anyhow!("User not found: {}", id))?;

        user.username = req.username;
        user.password = req.password;
        user.status = req.status;
        user.last_login = req.last_login;

        let user_id = user.id;
        match data.items.get(&user_id) {
            None => {
                anyhow::bail!("User not found: {}", id)
            }
            Some(user) => Ok(user.clone()),
        }
    }

    async fn delete_user(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("User not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
