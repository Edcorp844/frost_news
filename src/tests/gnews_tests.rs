#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_top_headlines_success() {
        
        let mock_server = MockServer::start().await;

        let mock_response = serde_json::json!({
            "total_articles": 1,
            "articles": [
                {
                    "title": "Test Article",
                    "description": "This is a test",
                    "content": "Full content here",
                    "url": "https://test.com",
                    "image": "https://test.com/image.jpg",
                    "publishedAt": "2024-01-01T00:00:00Z",
                    "source": {
                        "name": "Test Source",
                        "url": "https://test.com"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/top-headlines"))
            .and(query_param("token", "fake_key"))
            .and(query_param("lang", "en"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        // 4. Initialize client pointing to mock server
        let mut client = GNewsClient::new("fake_key");
        // Override base_url to point to our mock server
        client.base_url = Url::parse(&mock_server.uri()).unwrap().join("/").unwrap();

        // 5. Execute the function
        let result = client
            .fetch_top_headlines(None, Some("en".to_string()), None, Some(1))
            .await;

        // 6. Assertions
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.total_articles, 1);
        assert_eq!(response.articles[0].title, "Test Article");
    }

    #[tokio::test]
    async fn test_gnews_400_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(400)) // Simulate Bad Request
            .mount(&mock_server)
            .await;

        let mut client = GNewsClient::new("invalid_key");
        client.base_url = Url::parse(&mock_server.uri()).unwrap().join("/").unwrap();

        let result = client.fetch_top_headlines(None, None, None, None).await;

        // This should be an Err because perform_request returns Result<..., reqwest::Error>
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_error(){
        let gnews_api_key = std::env::var("GNEWS_API_KEY").expect("NEWS_API_KEY not set in .env");

        let client  = GNewsClient::new(gnews_api_key);
        let result = client.fetch_top_headlines(None, None, None, None).await;
        print!({}, result.articles)
    }
}
