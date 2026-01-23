use crate::gnews::data_structures::{GNewsArticle, GNewsSource};
use crate::gnews::gnews_client::GNewsClient;
use crate::news_api::data_structures::{NewsAPIArticle, Source};
use crate::news_api::news_api_client::NewsAPIClient;
use crate::newsdata::datap_structures::NewsDataArticle;
use crate::newsdata::newsdata_client::{NewsEndpoint, NewsdataClient};
use crate::types::news_article::NewsArticle;
use crate::types::request_parameters::RequestParameters;
use crate::utils::generator::Generator;
use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait NewsClient {
    fn name(&self) -> &'static str;
    fn supported_languages(&self) -> Vec<(&'static str, &'static str)>;
    fn format_language_code(&self, code: &str) -> String;
    async fn fetch_general(
        &self,
        parameters: RequestParameters,
    ) -> Result<Vec<Arc<dyn NewsArticle>>, String>;
    async fn fetch_business(&self) -> Result<Vec<Arc<dyn NewsArticle>>, String>;
    async fn fetch_testnews(&self,  parameters: RequestParameters) -> Result<Vec<Arc<dyn NewsArticle>>, String>;
}

#[async_trait]
impl NewsClient for NewsAPIClient {
    fn name(&self) -> &'static str {
        "NewsAPI"
    }

    fn supported_languages(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("", "All Languages"),
            ("ar", "Arabic"),
            ("de", "German"),
            ("en", "English"),
            ("es", "Spanish"),
            ("fr", "French"),
            ("he", "Hebrew"),
            ("it", "Italian"),
            ("nl", "Dutch"),
            ("no", "Norwegian"),
            ("pt", "Portuguese"),
            ("ru", "Russian"),
            ("sv", "Swedish"),
            ("ud", "Urdu"),
            ("zh", "Chinese"),
        ]
    }

    fn format_language_code(&self, code: &str) -> String {
        code.to_string()
    }

    async fn fetch_general(
        &self,
        parameters: RequestParameters,
    ) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .fetch_top_headlines(
                parameters.get_country(),
                parameters.get_category(),
                parameters.get_query(),
                parameters.get_language(),
                parameters.get_page_size(),
                parameters.get_page(),
            )
            .await;

        match response {
            Ok(res) => {
                let articles = res.articles.unwrap_or_default();
                let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
                    .into_iter()
                    .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
                    .collect();
                Ok(arc_articles)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn fetch_business(&self) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .fetch_top_headlines(
                Some("us".into()),
                Some("business".into()),
                None,
                None,
                Some(100),
                None,
            )
            .await;

        match response {
            Ok(res) => {
                let articles = res.articles.unwrap_or_default();
                let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
                    .into_iter()
                    .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
                    .collect();
                Ok(arc_articles)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn fetch_testnews(&self,  _parameters: RequestParameters) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let mut articles = Vec::new();
        for i in 0..100 {
            let article = NewsAPIArticle {
                source: Source {
                    id: Some("news_id".into()),
                    name: "Source Name".into(),
                },
                author: Some("Authour Name".into()),
                title: "This is an example of a news title. It could even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                description: Some("This is an example of a news Description. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into()),
                url: format!("https://test_news_urls.com{i}"),
                url_to_image: Some("https://test_news_urls.com".into(),),
                published_at: Generator::generate_date(),
                content: Some("This is an example of a news Content. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into()),
            };
            articles.push(article);
        }

        let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
            .into_iter()
            .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
            .collect();
        Ok(arc_articles)
    }
}

#[async_trait]
impl NewsClient for GNewsClient {
    fn name(&self) -> &'static str {
        "NewsAPI"
    }

    fn supported_languages(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("ar", "Arabic"),
            ("bn", "Bengali"),
            ("bg", "Bulgarian"),
            ("ca", "Catalan"),
            ("zh", "Chinese"),
            ("cs", "Czech"),
            ("nl", "Dutch"),
            ("en", "English"),
            ("et", "Estonian"),
            ("fi", "Finnish"),
            ("fr", "French"),
            ("de", "German"),
            ("el", "Greek"),
            ("gu", "Gujarati"),
            ("he", "Hebrew"),
            ("hi", "Hindi"),
            ("hu", "Hungarian"),
            ("id", "Indonesian"),
            ("it", "Italian"),
            ("ja", "Japanese"),
            ("ko", "Korean"),
            ("lv", "Latvian"),
            ("lt", "Lithuanian"),
            ("ml", "Malayalam"),
            ("mr", "Marathi"),
            ("no", "Norwegian"),
            ("pl", "Polish"),
            ("pt", "Portuguese"),
            ("pa", "Punjabi"),
            ("ro", "Romanian"),
            ("ru", "Russian"),
            ("sk", "Slovak"),
            ("sl", "Slovenian"),
            ("es", "Spanish"),
            ("sv", "Swedish"),
            ("ta", "Tamil"),
            ("te", "Telugu"),
            ("th", "Thai"),
            ("tr", "Turkish"),
            ("uk", "Ukrainian"),
            ("vi", "Vietnamese"),
        ]
    }

    fn format_language_code(&self, code: &str) -> String {
        code.to_string()
    }

    async fn fetch_general(
        &self,
        parameters: RequestParameters,
    ) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .fetch_top_headlines(
                parameters.get_category(),
                parameters.get_language(),
                parameters.get_country(),
                parameters.get_page_size(),
            )
            .await;

        match response {
            Ok(res) => {
                let articles = res.articles.unwrap_or_default();
                let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
                    .into_iter()
                    .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
                    .collect();
                Ok(arc_articles)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn fetch_business(&self) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .fetch_top_headlines(Some("business".into()), None, Some("us".into()), None)
            .await;

        match response {
            Ok(res) => {
                let articles = res.articles.unwrap_or_default();
                let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
                    .into_iter()
                    .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
                    .collect();
                Ok(arc_articles)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn fetch_testnews(&self,   _parameters: RequestParameters) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let mut articles = Vec::new();
        for _ in 0..100 {
            let article = GNewsArticle {
                id: "article_id".into(),
                title:  "This is an example of a news title. It could even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                description: "This is an example of a news Description. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                content: "This is an example of a news Content. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                url: "https://testUrl.com".into(),
                image:  "https://testUrl.com".into(),
                published_at: Generator::generate_date(),
                language: "en".into(),
                country: None,
                source: GNewsSource{ id: "Source id".into(), name: "Source name".into(), url: "https://SourceUrls.com".into(), country: None },
            };
            articles.push(article);
        }

        let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
            .into_iter()
            .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
            .collect();
        Ok(arc_articles)
    }
}

#[async_trait]
impl NewsClient for NewsdataClient {
    fn name(&self) -> &'static str {
        "NewsData"
    }

    fn supported_languages(&self) -> Vec<(&'static str, &'static str)> {
        todo!()
    }

    fn format_language_code(&self, code: &str) -> String {
        code.to_lowercase()
    }

    async fn fetch_general(
        &self,
        parameters: RequestParameters,
    ) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .endpoint(NewsEndpoint::Market)
            .language("en")
            .fetch()
            .await
            .map_err(|e| {
                eprintln!("API Request failed: {}", e);
                e.to_string()
            })?;

        // Now deserialization succeeds!
        let articles: Vec<Arc<dyn NewsArticle>> = response
            .results
            .into_iter()
            .map(|article_data| {
                // Arc::new(article_data) works now because NewsDataArticle
                // is compatible with the JSON strings.
                Arc::new(article_data) as Arc<dyn NewsArticle>
            })
            .collect();

        Ok(articles)
    }

    async fn fetch_business(&self) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let response = self
            .endpoint(NewsEndpoint::Market)
            .language("en")
            .video(true)
            .fetch()
            .await
            .map_err(|e| {
                eprintln!("API Request failed: {}", e);
                e.to_string()
            })?;

        let articles: Vec<Arc<dyn NewsArticle>> = response
            .results
            .into_iter()
            .map(|article_data| {
                // Arc::new(article_data) works now because NewsDataArticle
                // is compatible with the JSON strings.
                Arc::new(article_data) as Arc<dyn NewsArticle>
            })
            .collect();

        Ok(articles)
    }

    async fn fetch_testnews(&self,   _parameters: RequestParameters) -> Result<Vec<Arc<dyn NewsArticle>>, String> {
        let mut articles = Vec::new();
        for _ in 0..100 {
            let article = GNewsArticle {
                id: "article_id".into(),
                title:  "This is an example of a news title. It could even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                description: "This is an example of a news Description. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                content: "This is an example of a news Content. It could also even be as long as it wants that really doesn't depend on us but on the News Provider. We jsut have to render it right ".into(),
                url: "https://testUrl.com".into(),
                image:  "https://testUrl.com".into(),
                published_at: Generator::generate_date(),
                language: "en".into(),
                country: None,
                source: GNewsSource{ id: "Source id".into(), name: "Source name".into(), url: "https://SourceUrls.com".into(), country: None },
            };
            articles.push(article);
        }

        let arc_articles: Vec<Arc<dyn NewsArticle>> = articles
            .into_iter()
            .map(|a| Arc::new(a) as Arc<dyn NewsArticle>)
            .collect();
        Ok(arc_articles)
    }
}
