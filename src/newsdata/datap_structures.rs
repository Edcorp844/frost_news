use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsDataResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: i32,
    pub results: Vec<NewsDataArticle>,
    #[serde(rename = "nextPage")]
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsDataArticle {
    pub article_id: String,
    pub title: String,
    #[serde(rename = "link")]
    pub url: String,
    pub keywords: Option<Vec<String>>,
    pub creator: Option<Vec<String>>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    #[serde(rename = "pubDate")]
    pub published_at: String,
    pub image_url: Option<String>,
    pub source_id: String,
    pub source_url: Option<String>,
    pub source_icon: Option<String>,
    pub source_priority: i32,
    pub country: Vec<String>,
    pub category: Vec<String>,
    pub language: String,

    // AI and Advanced Insights
    pub ai_tag: serde_json::Value,
    pub sentiment: serde_json::Value,
    pub sentiment_stats: serde_json::Value,
    pub ai_region: serde_json::Value,
    pub ai_org: serde_json::Value,

    // Metadata and Specific Endpoints
    #[serde(rename = "pubDateTZ")]
    pub pub_date_tz: Option<String>,
    pub coin: Option<Vec<String>>, // Exclusive to Crypto endpoint
    pub duplicate: bool,
    #[serde(rename = "ai_summary")]
    pub ai_summary: serde_json::Value, // Paid users only
    pub datatype: String,
}

#[derive(Debug, thiserror::Error)]
pub enum NewsdataError {
    #[error("Bad Request: Parameter missing or invalid. {0}")]
    BadRequest(String),
    #[error("Unauthorized: API key is invalid or missing.")]
    Unauthorized,
    #[error("Forbidden: IP/Domain restricted by CORS policy.")]
    Forbidden,
    #[error("Not Found: The page or article does not exist.")]
    NotFound,
    #[error("Conflict: Parameter duplicate value detected.")]
    ParameterDuplicate,
    #[error("Unsupported Media Type: The request format is not supported.")]
    UnsupportedType,
    #[error("Unprocessable Entity: Semantic error in the request.")]
    UnprocessableEntity,
    #[error("Too Many Requests: Rate limit exceeded for your plan.")]
    RateLimitExceeded,
    #[error("Internal Server Error: An unexpected error occurred on the server.")]
    InternalServerError,
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Unknown Error: Status code {0}")]
    Unknown(u16),
}
