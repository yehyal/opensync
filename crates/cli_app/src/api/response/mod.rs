use serde::{Deserialize, Serialize};

pub mod login;
pub mod posts;
pub mod users;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
