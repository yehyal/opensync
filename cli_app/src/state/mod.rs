use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::{
    services::{post::InMemoryPostService, user::UserService},
    settings::Settings,
};

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub post_service: Arc<InMemoryPostService>,
    pub user_service: Arc<UserService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            post_service: Arc::new(InMemoryPostService::default()),
            user_service: Arc::new(UserService::default()),
        })
    }
}
