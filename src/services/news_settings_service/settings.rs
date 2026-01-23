use gtk::gio::{Settings, prelude::SettingsExt};

use crate::types::news_source::NewsSource;

#[derive(Debug, Clone)]
pub struct NewsServiceSettings {
    settings: Settings,
}

impl NewsServiceSettings {
    pub fn new(schema_id: &str) -> Self {
        let settings = Settings::new(schema_id);
        Self { settings }
    }

    // --- GETTERS ---
    pub fn country(&self) -> String {
        self.settings.string("country").to_string()
    }

    pub fn language(&self) -> String {
        self.settings.string("language").to_string()
    }

    pub fn news_source(&self) -> NewsSource {
        let source_str = self.settings.string("news-source");

        match source_str.as_str() {
            "NewsAPI" => NewsSource::NewsAPI,
            "GNews" => NewsSource::GNews,
            "NewsData" => NewsSource::NewsData,
            _ => NewsSource::NewsAPI, // This is your "Default" fallback
        }
    }

    // --- SETTERS (Overriding Defaults) ---

    pub fn set_country(&self, value: &str) {
        self.settings
            .set_string("country", value)
            .expect("Failed to set country setting");
    }

    pub fn set_langauge(&self, value: &str) {
        self.settings
            .set_string("language", value)
            .expect("Failed to set country setting");
    }

    pub fn set_news_source(&self, source: NewsSource) {
        let value = match source {
            NewsSource::NewsAPI => "NewsAPI",
            NewsSource::GNews => "GNews",
            NewsSource::NewsData => "NewsData",
        };
        self.settings
            .set_string("news-source", value)
            .expect("Failed to save news source to GSettings");
    }

    // --- RESETTING (Back to Defaults) ---

    pub fn reset_country(&self) {
        self.settings.reset("country");
    }

    pub fn reset_language(&self) {
        self.settings.reset("language");
    }

    pub fn reset_all(&self) {
        self.settings.reset("country");
        self.settings.reset("news-source");
        self.settings.reset("language");
        // Or loop through keys if you have many
    }
}
