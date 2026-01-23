use std::{collections::BTreeMap, sync::Arc};

use relm4::{Component, ComponentSender};

use crate::{
    gnews::gnews_client::GNewsClient,
    news_api::news_api_client::NewsAPIClient,
    newsdata::newsdata_client::NewsdataClient,
    services::news_settings_service::settings::NewsServiceSettings,
    types::{
        news_article::NewsArticle, news_category::NewsSection, news_client::NewsClient,
        news_handler::NewsHandler, news_source::NewsSource, request_parameters::RequestParameters,
    },
};

#[derive(Debug, Clone)]
pub struct NewsFetchService {
    newsapi_api_key: String,
    gnews_api_key: String,
    newsdata_api_key: String,
    settings: NewsServiceSettings,
    request_parameters: RequestParameters,
}

impl NewsFetchService {
    pub fn new(
        newsapi_api_key: String,
        gnews_api_key: String,
        newsdata_api_key: String,
        settings: NewsServiceSettings,
    ) -> Self {
        Self {
            newsapi_api_key,
            gnews_api_key,
            newsdata_api_key,
            settings,
            request_parameters: RequestParameters::new(),
        }
    }

    pub fn get_settings(&self) -> NewsServiceSettings {
        self.settings.clone()
    }

    fn get_client(&self, source: NewsSource) -> Box<dyn NewsClient> {
        match source {
            NewsSource::GNews => Box::new(GNewsClient::new(self.gnews_api_key.clone())),
            NewsSource::NewsAPI => Box::new(NewsAPIClient::new(self.newsapi_api_key.clone())),
            NewsSource::NewsData => Box::new(NewsdataClient::new(self.newsdata_api_key.clone())),
        }
    }

    pub fn sync_parameters(&mut self) {
        let settings = self.settings.clone();

        self.request_parameters = self
            .request_parameters
            .clone()
            .language(settings.language())
            .country(settings.country());
    }

    pub fn fetch_news<T: Component>(
        &mut self,
        category: NewsSection,
        page: i32,
        sender: ComponentSender<T>,
    ) where
        T::Input: NewsHandler,
    {
        let client = self.get_client(self.get_settings().news_source().clone());
        self.request_parameters = self.request_parameters.clone().page(page);

        match category {
            NewsSection::General => self.fetch_general_news(client, sender),
            NewsSection::Business => self.fetch_business_news(client, sender),
            NewsSection::Health => return,
            NewsSection::Entertainment => return,
            NewsSection::Technology => return,
            NewsSection::Science => return,
            NewsSection::Sports => return,
        }
    }

    fn fetch_general_news<T: Component>(
        &mut self,
        client: Box<dyn NewsClient>,
        sender: ComponentSender<T>,
    ) where
        T::Input: NewsHandler,
    {
        self.sync_parameters();
        let params = self.request_parameters.clone();
        gtk::glib::spawn_future_local(async move {
            match client.fetch_testnews(params).await {
                Ok(articles) => {
                    let mut grouped: BTreeMap<String, Vec<Arc<dyn NewsArticle>>> = BTreeMap::new();
                    let time_organiser = crate::utils::time_organizer::UITimeOrganiser::new();

                    for article in articles {
                        let bucket = time_organiser.time_bucket_key(&article.published_at());
                        let bucket_key = time_organiser.categorize_by_relative_time(bucket);
                        grouped.entry(bucket_key).or_default().push(article);
                    }

                    sender.input(T::Input::on_news_received(grouped));
                }
                Err(e) => {
                    sender.input(T::Input::on_error(e.to_string()));
                }
            }
        });
    }

    fn fetch_business_news<T: Component>(
        &self,
        client: Box<dyn NewsClient>,
        sender: ComponentSender<T>,
    ) where
        T::Input: NewsHandler,
    {
        gtk::glib::spawn_future_local(async move {
            match client.fetch_business().await {
                Ok(articles) => {
                    let mut grouped: BTreeMap<String, Vec<Arc<dyn NewsArticle>>> = BTreeMap::new();
                    let time_organiser = crate::utils::time_organizer::UITimeOrganiser::new();

                    for article in articles {
                        let bucket = time_organiser.time_bucket_key(&article.published_at());
                        let bucket_key = time_organiser.categorize_by_relative_time(bucket);
                        grouped.entry(bucket_key).or_default().push(article);
                    }

                    sender.input(T::Input::on_news_received(grouped));
                }
                Err(e) => {
                    sender.input(T::Input::on_error(e.to_string()));
                }
            }
        });
    }
}
