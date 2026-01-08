use reqwest::Client;
use url::Url;

use crate::newsdata::datap_structures::{NewsdataError, NewsDataResponse};
pub enum NewsEndpoint {
    Latest,
    Crypto,
    Archive,
    Sources,
    Market,
}

impl NewsEndpoint {
    pub fn path(&self) -> &'static str {
        match self {
            NewsEndpoint::Latest => "latest",
            NewsEndpoint::Crypto => "crypto",
            NewsEndpoint::Archive => "archive",
            NewsEndpoint::Sources => "sources",
            NewsEndpoint::Market => "market"
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewsdataClient {
    api_token: String,
    base_url: Url,
    client: Client,
}

impl NewsdataClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
            base_url: Url::parse("https://newsdata.io/api/1/").unwrap(),
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

        println!("🌐 NewsData URL: {}", url);
        Ok(url)
    }

    async fn perform_request(&self, url: Url) -> Result<NewsDataResponse, NewsdataError> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "FrostNews/1.0")
            .send()
            .await?;

        let status = response.status();
        println!("📡 NewsData HTTP Status: {}", status);

        match status.as_u16() {
            200 => {
                let body = response.json::<NewsDataResponse>().await?;
                print!("{:?}", body);
                Ok(body)
            }

            400 => {
                let error_body = response.json::<serde_json::Value>().await.ok();
                let msg = error_body
                    .and_then(|v| v["results"]["message"].as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "Parameter missing or invalid".to_string());
                Err(NewsdataError::BadRequest(msg))
            }
            401 => Err(NewsdataError::Unauthorized),
            403 => Err(NewsdataError::Forbidden),
            404 => Err(NewsdataError::NotFound),
            409 => Err(NewsdataError::ParameterDuplicate),
            415 => Err(NewsdataError::UnsupportedType),
            422 => Err(NewsdataError::UnprocessableEntity),
            429 => Err(NewsdataError::RateLimitExceeded),
            500 => Err(NewsdataError::InternalServerError),
            code => Err(NewsdataError::Unknown(code)),
        }
    }


    pub fn endpoint(&self, endpoint: NewsEndpoint) -> NewsRequestBuilder {
        NewsRequestBuilder::new(self, endpoint)
    }
}

pub struct NewsRequestBuilder<'a> {
    client: &'a NewsdataClient,
    endpoint: NewsEndpoint,
    params: Vec<(String, String)>,
}

impl<'a> NewsRequestBuilder<'a> {
    fn new(client: &'a NewsdataClient, endpoint: NewsEndpoint) -> Self {
        Self { client, endpoint, params: Vec::new() }
    }

    // --- Search & Keywords ---
    pub fn q(mut self, val: impl Into<String>) -> Self { self.params.push(("q".into(), val.into())); self }
    pub fn q_in_title(mut self, val: impl Into<String>) -> Self { self.params.push(("qInTitle".into(), val.into())); self }
    pub fn q_in_meta(mut self, val: impl Into<String>) -> Self { self.params.push(("qInMeta".into(), val.into())); self }
    
    // --- Filters ---
    pub fn country(mut self, val: impl Into<String>) -> Self { self.params.push(("country".into(), val.into())); self }
    pub fn category(mut self, val: impl Into<String>) -> Self { self.params.push(("category".into(), val.into())); self }
    pub fn language(mut self, val: impl Into<String>) -> Self { self.params.push(("language".into(), val.into())); self }
    pub fn domain_url(mut self, val: impl Into<String>) -> Self { self.params.push(("domainurl".into(), val.into())); self }
    
    // --- Exclusions ---
    pub fn exclude_country(mut self, val: impl Into<String>) -> Self { self.params.push(("excludecountry".into(), val.into())); self }
    pub fn exclude_category(mut self, val: impl Into<String>) -> Self { self.params.push(("excludecategory".into(), val.into())); self }
    pub fn exclude_domain(mut self, val: impl Into<String>) -> Self { self.params.push(("excludedomain".into(), val.into())); self }
    pub fn exclude_language(mut self, val: impl Into<String>) -> Self { self.params.push(("excludelanguage".into(), val.into())); self }
    pub fn exclude_field(mut self, val: impl Into<String>) -> Self { self.params.push(("excludefield".into(), val.into())); self }

    // --- Crypto & Market Specific ---
    pub fn coin(mut self, val: impl Into<String>) -> Self { self.params.push(("coin".into(), val.into())); self }
    pub fn symbol(mut self, val: impl Into<String>) -> Self { self.params.push(("symbol".into(), val.into())); self }

    // --- Time & Archive (YYYY-MM-DD) ---
    pub fn from_date(mut self, val: impl Into<String>) -> Self { self.params.push(("from_date".into(), val.into())); self }
    pub fn to_date(mut self, val: impl Into<String>) -> Self { self.params.push(("to_date".into(), val.into())); self }
    pub fn timeframe(mut self, hours: u32) -> Self { self.params.push(("timeframe".into(), hours.to_string())); self }
    pub fn timezone(mut self, val: impl Into<String>) -> Self { self.params.push(("timezone".into(), val.into())); self }

    // --- AI & Advanced Metadata ---
    pub fn sentiment(mut self, val: &str) -> Self { self.params.push(("sentiment".into(), val.into())); self }
    pub fn region(mut self, val: impl Into<String>) -> Self { self.params.push(("region".into(), val.into())); self }
    pub fn tag(mut self, val: impl Into<String>) -> Self { self.params.push(("tag".into(), val.into())); self }
    pub fn organization(mut self, val: impl Into<String>) -> Self { self.params.push(("organization".into(), val.into())); self }
    pub fn priority_domain(mut self, val: &str) -> Self { self.params.push(("prioritydomain".into(), val.into())); self }

    // --- Boolean Flags (Serialized as 1/0) ---
    pub fn full_content(mut self, val: bool) -> Self { self.params.push(("full_content".into(), (val as i32).to_string())); self }
    pub fn image(mut self, val: bool) -> Self { self.params.push(("image".into(), (val as i32).to_string())); self }
    pub fn video(mut self, val: bool) -> Self { self.params.push(("video".into(), (val as i32).to_string())); self }
    pub fn remove_duplicate(mut self, val: bool) -> Self { self.params.push(("removeduplicate".into(), (val as i32).to_string())); self }

    // --- Pagination & Sorting ---
    pub fn size(mut self, val: u32) -> Self { self.params.push(("size".into(), val.to_string())); self }
    pub fn page(mut self, val: impl Into<String>) -> Self { self.params.push(("page".into(), val.into())); self }
    pub fn sort(mut self, val: &str) -> Self { self.params.push(("sort".into(), val.into())); self }

    // --- Fetch ---
    pub async fn fetch(self) -> Result<NewsDataResponse, NewsdataError> {
        let mut url = self.client.base_url.join(self.endpoint.path()).unwrap();
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("apikey", &self.client.api_token);
            for (k, v) in self.params {
                query.append_pair(&k, &v);
            }
        }
        self.client.perform_request(url).await
    }
}