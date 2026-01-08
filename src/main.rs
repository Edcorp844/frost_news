use crate::{
    pages::app_window::FrostNewsAPPWindow,
    services::{
        news_service::fetch_service::NewsFetchService,
        news_settings_service::settings::NewsServiceSettings,
    },
    types::{app_config::AppConfig, cache::ImageCache},
};
use adw::prelude::*;
use dotenv::dotenv;
use gtk::{gdk::Display, gio, glib};

mod gnews;
mod news_api;
mod newsdata;
mod pages;
mod services;
mod types;
mod ui_build_herlper_functions;
mod widgets;

const APP_ID: &'static str = "com.example.frostnews";

#[tokio::main]
async fn main() -> glib::ExitCode {
    gio::resources_register_include!("frostnews.gresource").expect("Resources failed");
    dotenv().ok();

    //for windows sake
    std::env::set_var("GSETTINGS_SCHEMA_DIR", "./data");

    let news_api_api_key = std::env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set in .env");
    let gnews_api_key = std::env::var("GNEWS_API_KEY").expect("GNEWS_API_KEY not set in .env");
    let newsdata_api_key =
        std::env::var("NEWSDATA_API_KEY").expect("NEWSDATA_API_KEY not set in .env");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(glib::clone!(
        #[strong]
        news_api_api_key,
        #[strong]
        gnews_api_key,
        #[strong]
        newsdata_api_key,
        move |app| {
            load_css();

            let window = FrostNewsAPPWindow::new();
            let split_view = window.get_split_view();

            let news_service_settings = NewsServiceSettings::new(APP_ID);
            let news_fetch_service = NewsFetchService::new(
                news_api_api_key.clone(),
                gnews_api_key.clone(),
                newsdata_api_key.clone(),
                news_service_settings,
                ImageCache::new(),
                &split_view,
            );

            let app_config = AppConfig { news_fetch_service };

            window.config(app_config);
            window.initialise();

            window.set_application(Some(app));
            window.present();
        }
    ));

    app.run()
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_path("resources/style/style.css");

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
