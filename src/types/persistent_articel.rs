use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct PersistentArticle {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub published_at: String,
    pub image_url: String,
    pub visit_time: i64,
}

impl PersistentArticle {
    pub fn auto_create(
        title: String,
        url: String,
        description: Option<String>,
        content: Option<String>,
        published_at: String,
        image_url: String,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let id = hex::encode(hasher.finalize());
        let now = chrono::Utc::now().timestamp();

        Self {
            id,
            title,
            url,
            description,
            content,
            published_at,
            image_url,
            visit_time: now,
        }
    }
}
