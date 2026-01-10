use std::{collections::BTreeMap, sync::Arc};

use adw::NavigationSplitView;
use chrono::{DateTime, Utc};

use crate::{gnews::gnews_client::GNewsClient, news_api::news_api_client::NewsAPIClient, newsdata::newsdata_client::NewsdataClient, pages::{businessnews_page::BusinessNewsPage, healthnews_page::HealthNewsPage, top_headlines_page::TopHeadlinesPage}, services::news_settings_service::settings::NewsServiceSettings, types::{cache::ImageCache, news_article::NewsArticle, news_category::NewsCategory, news_client::NewsClient, news_source::NewsSource}, ui_build_herlper_functions::ui_content_organiser::time_organiser::UITimeOrganiser};


#[derive(Debug, Clone)]
pub struct NewsFetchService {
    newsapi_api_key: String,
    gnews_api_key: String,
    newsdata_api_key: String,
    settings: NewsServiceSettings,
    cache: ImageCache,
    parent: NavigationSplitView,
}

impl NewsFetchService {
    pub fn new(
        newsapi_api_key: String,
        gnews_api_key: String,
        newsdata_api_key: String,
        settings: NewsServiceSettings,
        cache: ImageCache,
        parent: &NavigationSplitView,
    ) -> Self {
        Self {
            newsapi_api_key,
            gnews_api_key,
            newsdata_api_key,
            settings,
            cache,
            parent: parent.clone(),
        }
    }

    pub fn get_settings(&self)->NewsServiceSettings{
        self.settings.clone()
    }

    pub fn fetch_news(&self, category: NewsCategory, source: NewsSource) {
        let client: Box<dyn NewsClient> = match source {
            NewsSource::GNews => {
                Box::new(GNewsClient::new(self.gnews_api_key.clone())) as Box<dyn NewsClient>
            }
            NewsSource::NewsAPI => {
                Box::new(NewsAPIClient::new(self.newsapi_api_key.clone())) as Box<dyn NewsClient>
            }
            NewsSource::NewsData => {
                 Box::new(NewsdataClient::new(self.newsdata_api_key.clone())) as Box<dyn NewsClient>
            }
        };

        match category {
            NewsCategory::General => self.run_fetch_general(client),
            NewsCategory::Business => self.fetch_business(client),
            NewsCategory::Health => self.fetch_health(client),
            NewsCategory::Entertainment => return,
            NewsCategory::Technology => return,
            NewsCategory::Science => return,
            NewsCategory::Sports => return,
        }
    }

    fn run_fetch_general(&self, client: Box<dyn NewsClient>) {
        let page = TopHeadlinesPage::new(self.cache.clone(), &self.parent.clone());
        page.show_loading();

        glib::spawn_future_local(async move {
            if let Ok(mut articles) = client.fetch_general().await {
                let top_articles: Vec<Arc<dyn NewsArticle>> = articles
                    .drain(0..15.min(articles.len()))
                    .collect::<Vec<Arc<dyn NewsArticle>>>();

                page.add_top_headlines(&top_articles);

                let mut grouped: BTreeMap<DateTime<Utc>, Vec<Arc<dyn NewsArticle>>> =
                    BTreeMap::new();
                let time_organiser = UITimeOrganiser::new();

                for article in articles {
                    let published = article.published_at();
                    let bucket = time_organiser.time_bucket_key(&published);
                    grouped.entry(bucket).or_default().push(article);
                }

                page.add_articles_sections(&grouped);
                page.show_content();
            }
        });
    }

    fn fetch_business(&self, client: Box<dyn NewsClient>) {
        let page = BusinessNewsPage::new(self.cache.clone(), &self.parent);

        page.show_content();
    }

     fn fetch_health(&self, client: Box<dyn NewsClient>) {
        let page = HealthNewsPage::new(self.cache.clone(), &self.parent);

        page.show_content();
    }
}
