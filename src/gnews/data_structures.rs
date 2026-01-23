#[derive(serde::Deserialize, Debug, Clone)]

pub struct GNewsArticle {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub url: String,
    pub image: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "lang")]
    pub language: String,
    pub country: Option<String>,
    pub source: GNewsSource,
}

#[derive(serde::Deserialize, Debug, Clone)]

pub struct GNewsSource {
    pub id: String,
    pub name: String,
    pub url: String,
    pub country: Option<String>
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GNewsResponse {
     #[serde(rename = "totalArticles")]
    pub total_articles: i32,
    pub articles: Option<Vec<GNewsArticle>>,
}

#[derive(Debug, thiserror::Error)]
pub enum NewsError {
    #[error("Bad Request: The request was malformed.")]
    BadRequest,
    #[error("Unauthorized: Invalid or missing API key.")]
    Unauthorized,
    #[error("Forbidden: Daily quota reached. Resets at 00:00 UTC.")]
    Forbidden,
    #[error("Too Many Requests: Rate limit exceeded.")]
    TooManyRequests,
    #[error("Internal Server Error: Something went wrong on the server.")]
    InternalServerError,
    #[error("Service Unavailable: Server is offline for maintenance.")]
    ServiceUnavailable,
    #[error("Network Error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Application Error: {0}")]
    Unknown(u16),
}