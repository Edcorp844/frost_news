use reqwest::Client;
use url::Url;

use crate::gnews::data_structures::{GNewsResponse, NewsError};

#[derive(Debug, Clone)]
pub struct GNewsClient {
    api_token: String,
    base_url: Url,
    client: Client,
}

impl GNewsClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
            base_url: Url::parse("https://gnews.io/api/v4/").unwrap(),
            client: Client::new(),
        }
    }

    fn build_url(
        &self,
        endpoint: &str,
        params: &[(&str, Option<String>)],
    ) -> Result<Url, url::ParseError> {
        let mut url = self.base_url.join(endpoint)?;

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in params {
                if let Some(v) = value {
                    let trimmed = v.trim();
                    if !trimmed.is_empty() {
                        pairs.append_pair(key, trimmed);
                    }
                }
            }
            
            pairs.append_pair("apikey", &self.api_token);
        }

        println!("🌐 GNews URL: {}", url);
        Ok(url)
    }

    async fn perform_request(&self, url: Url) -> Result<GNewsResponse, NewsError> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "FrostNews/1.0")
            .send()
            .await?;

        let status = response.status();
        println!("📡 GNews HTTP Status: {}", status);

        match status.as_u16() {
            200 => {
                let body = response.json::<GNewsResponse>().await?;
                print!("{:?}", body);
                Ok(body)
            }
            400 => Err(NewsError::BadRequest),
            401 => Err(NewsError::Unauthorized),
            403 => Err(NewsError::Forbidden),
            429 => Err(NewsError::TooManyRequests),
            500 => Err(NewsError::InternalServerError),
            503 => Err(NewsError::ServiceUnavailable),
            other => Err(NewsError::Unknown(other)),
        }
    }

    /// Fetch top headlines from GNews
    pub async fn fetch_top_headlines(
        &self,
        category: Option<String>,
        lang: Option<String>,
        country: Option<String>,
        max_results: Option<i32>,
    ) -> Result<GNewsResponse, NewsError> {
        let url = self
            .build_url(
                "top-headlines",
                &[
                    ("category", category),
                    ("lang", lang),
                    ("country", country),
                    ("max", max_results.map(|v| v.to_string())),
                ],
            )
            .unwrap();

        self.perform_request(url).await
    }

    /// Search for specific keywords
    pub async fn search(
        &self,
        query: String,
        lang: Option<String>,
        country: Option<String>,
        max_results: Option<i32>,
    ) -> Result<GNewsResponse, NewsError> {
        let url = self
            .build_url(
                "search",
                &[
                    ("q", Some(query)),
                    ("lang", lang),
                    ("country", country),
                    ("max", max_results.map(|v| v.to_string())),
                ],
            )
            .unwrap();

        self.perform_request(url).await
    }
}
