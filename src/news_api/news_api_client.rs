use reqwest::Client;
use url::Url;

use crate::news_api::data_structures::{NewsAPIArticle, NewsAPIResponse, NewsAPISource};

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
    ) -> Result<NewsAPIResponse<T>, reqwest::Error> {
        println!("🌐 Making request to: {}", url);

        let response = self
            .client
            .get(url)
            .header("User-Agent", "FrostNews/1.0")
            .send()
            .await?;
        let status = response.status();

        println!("📡 HTTP Status: {}", status);

        let bytes = response.bytes().await?;

        let parsed = serde_json::from_slice::<NewsAPIResponse<T>>(&bytes).map_err(|e| {
            println!("❌ Decoding error: {e}");
            if let Ok(json) = String::from_utf8(bytes.to_vec()) {
                println!("🔍 JSON was: {json}");
            }
            e
        });

        Ok(parsed.unwrap())
    }

    pub async fn fetch_top_headlines(
        &self,
        country: Option<String>,
        category: Option<String>,
        query: Option<String>,
        language: Option<String>,
        page_size: Option<i32>,
        page: Option<i32>,
    ) -> Result<NewsAPIResponse<NewsAPIArticle>, reqwest::Error> {
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
    ) -> Result<NewsAPIResponse<NewsAPIArticle>, reqwest::Error> {
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
    ) -> Result<NewsAPIResponse<NewsAPISource>, reqwest::Error> {
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
