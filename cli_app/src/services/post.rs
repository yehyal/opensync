use anyhow::Ok;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::model::{Post, PostStatus};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreatePostDTO {
    pub user_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Deserialize)]
pub struct UpdatePostDTO {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}

pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostDTO) -> anyhow::Result<Post>;
    async fn update_post(&self, id: i64, req: UpdatePostDTO) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        if let Some(post) = data.items.get(&id) {
            Ok((*post).clone())
        } else {
            anyhow::bail!("Post not found: {}", id)
        }
    }

    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        for (_id, post) in data.items.iter() {
            if post.slug == slug {
                return Ok(post.clone());
            }
        }
        anyhow::bail!("Post not found: {}", slug)
    }

    async fn create_post(&self, req: CreatePostDTO) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        data.counter += 1;

        let timestamp = chrono::offset::Utc::now();

        let post = Post {
            id: data.counter,
            author_id: req.user_id,
            slug: req.slug,
            title: req.title,
            content: req.content,
            status: req.status,
            created_at: timestamp,
            updated_at: timestamp,
        };

        data.items.insert(post.id, post);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn update_post(&self, id: i64, req: UpdatePostDTO) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        let post = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::anyhow!("Post not found: {}", id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;
        post.updated_at = chrono::offset::Utc::now();

        Ok((*post).clone())
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
