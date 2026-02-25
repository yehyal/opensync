use std::sync::Arc;

use anyhow::Ok;
use arc_swap::ArcSwap;

use crate::{services::post::InMemoryPostService, settings::Settings};

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub post_service: Arc<InMemoryPostService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            post_service: Arc::new(InMemoryPostService::default()),
        })
    }
}
