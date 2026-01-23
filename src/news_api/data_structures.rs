use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::reqwest_error_extension::ReqwestErrorExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsAPIResponse<T> {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: Option<i32>,
    pub articles: Option<Vec<T>>,
    pub sources: Option<Vec<NewsAPISource>>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewsAPIArticle {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewsAPISource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub category: String,
    pub language: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewsAPIError {
    pub code: Option<String>,
    pub message: Option<String>,
}



#[derive(Error, Debug)]
pub enum NewsAPICusteomError {
   #[error("{}", .0.to_user_friendly_message())]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{message}")]
    Api {
        code: String,
        message: String,
    },

    #[error("Unknown error occurred")]
    Unknown,
}
