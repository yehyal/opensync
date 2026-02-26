use crate::model::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct SingleUserResponse {
    pub data: User,
}
