use reqwest::Client;
use url::Url;

use crate::news_api::data_structures::{
    NewsAPIArticle, NewsAPICusteomError, NewsAPIError, NewsAPIResponse, NewsAPISource
};

#[derive(Clone)]
pub struct NewsAPIClient {
    api_key: String,
    base_url: Url,
    client: Client,
}

impl NewsAPIClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        let base_url = Url::parse("https://newsapi.org").unwrap();

        println!("🔑 NewsAPIClient initialized with key: {}…", &api_key[..8]);

        Self {
            api_key,
            base_url,
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
            pairs.append_pair("apiKey", &self.api_key);
        }

        println!("🌐 Final URL: {}", url);

        Ok(url)
    }

    async fn perform_request<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
    ) -> Result<NewsAPIResponse<T>, NewsAPICusteomError> {
        println!("🌐 Making request to: {}", url);

        // 1. Send the request (Wraps reqwest::Error automatically via #[from])
        let response = self
            .client
            .get(url)
            .header("User-Agent", "FrostNews/1.0")
            .send()
            .await?;

        let status = response.status();
        println!("📡 HTTP Status: {}", status);

        // 2. Get the raw bytes
        let bytes = response.bytes().await?;

        // 3. LOGIC: If status is not 200, try to parse our NewsAPIError struct
        if !status.is_success() {
            if let Ok(api_err) = serde_json::from_slice::<NewsAPIError>(&bytes) {
                return Err(NewsAPICusteomError::Api {
                    code: api_err.code.unwrap_or_default(),
                    message: api_err
                        .message
                        .unwrap_or_else(|| "Unknown API error".into()),
                });
            }
            return Err(NewsAPICusteomError::Unknown);
        }

        // 4. If status IS 200, parse the actual data
        let parsed = serde_json::from_slice::<NewsAPIResponse<T>>(&bytes).map_err(|e| {
            if let Ok(json) = String::from_utf8(bytes.to_vec()) {
                println!("🔍 Decoding error. Raw JSON: {json}");
            }
            e
        })?;

        Ok(parsed)
    }

    pub async fn fetch_top_headlines(
        &self,
        country: Option<String>,
        category: Option<String>,
        query: Option<String>,
        language: Option<String>,
        page_size: Option<i32>,
        page: Option<i32>,
    ) -> Result<NewsAPIResponse<NewsAPIArticle>, NewsAPICusteomError> {
        println!("📰 Fetching top headlines…");

        let url = self
            .build_url(
                "/v2/top-headlines",
                &[
                    ("country", country),
                    ("category", category),
                    ("q", query),
                    ("language", language),
                    ("pageSize", page_size.map(|v| v.to_string())),
                    ("page", page.map(|v| v.to_string())),
                ],
            )
            .unwrap();

        self.perform_request(url).await
    }

    pub async fn fetch_everything(
        &self,
        query: Option<String>,
        from: Option<String>,
        to: Option<String>,
        language: Option<String>,
        sort_by: Option<String>,
        page_size: Option<i32>,
        page: Option<i32>,
    ) -> Result<NewsAPIResponse<NewsAPIArticle>, NewsAPICusteomError> {
        println!("🔍 Fetching everything…");

        let url = self
            .build_url(
                "/v2/everything",
                &[
                    ("q", query),
                    ("from", from),
                    ("to", to),
                    ("language", language),
                    ("sortBy", sort_by),
                    ("pageSize", page_size.map(|v| v.to_string())),
                    ("page", page.map(|v| v.to_string())),
                ],
            )
            .unwrap();

        self.perform_request(url).await
    }

    pub async fn fetch_sources(
        &self,
        category: Option<String>,
        language: Option<String>,
        country: Option<String>,
    ) -> Result<NewsAPIResponse<NewsAPISource>, NewsAPICusteomError> {
        println!("📡 Fetching sources…");

        let url = self
            .build_url(
                "/v2/sources",
                &[
                    ("category", category),
                    ("language", language),
                    ("country", country),
                ],
            )
            .unwrap();

        self.perform_request(url).await
    }
}
