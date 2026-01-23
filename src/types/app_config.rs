use crate::services::news_service::fetch_service::NewsFetchService;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub news_fetch_service: NewsFetchService,
}
