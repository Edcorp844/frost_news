use crate::{services::{news_service::fetch_service::NewsFetchService, news_settings_service::settings::NewsServiceSettings}, types::cache::ImageCache};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub news_fetch_service: NewsFetchService,
}
