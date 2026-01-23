use adw::prelude::*;
use relm4::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

mod components;
mod data;
mod gnews;
mod news_api;
mod newsdata;
mod pages;
mod services;
mod types;
mod utils;

use crate::components::sidebar::{NavigationPage, SideBar, SidebarMessage};
use crate::pages::category_page::{CategoryPage, PageInput, PageOutput};
use crate::pages::history_page::{HistoryPage, HistoryPagePageOutput};
use crate::services::history_service::history::HistoryService;
use crate::services::news_service::fetch_service::NewsFetchService;
use crate::services::news_settings_service::settings::NewsServiceSettings;
use crate::services::workers::history_worker::HistoryWorker;
use crate::types::cache::ImageCache;
use crate::types::news_category::NewsSection;
use dotenv::dotenv;

const APP_ID: &'static str = "com.example.frostnews";

enum PageController {
    Category(Controller<CategoryPage>),
    History(Controller<HistoryPage>),
}

impl PageController {
    fn widget(&self) -> &adw::NavigationPage {
        match self {
            Self::Category(c) => c.widget(),
            Self::History(c) => c.widget(),
        }
    }
}

struct App {
    sidebar_visible: bool,
    fetch_service: NewsFetchService,
    image_cache: ImageCache,
    pages_cache: HashMap<String, PageController>,
    current_page_key: String,
    sidebar: Controller<SideBar>,
    history_worker: Arc<Controller<HistoryWorker>>,
}

#[derive(Debug)]
enum Msg {
    ToggleSidebar,
    ChangeSection(NewsSection),
    ChangeSectionPage(NavigationPage),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        adw::Window {
            set_title: Some("Frost News 2026"),
            set_default_size: (1400, 900),

            adw::OverlaySplitView {
                #[watch]
                set_show_sidebar: model.sidebar_visible,

                #[wrap(Some)]
                set_sidebar = model.sidebar.widget(),

                #[wrap(Some)]
                set_content = &adw::Bin {
                    #[watch]
                    set_child: model.pages_cache.get(&model.current_page_key).map(|c| c.widget()),
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let fetch_service = NewsFetchService::new(
            std::env::var("NEWS_API_KEY").unwrap_or_default(),
            std::env::var("GNEWS_API_KEY").unwrap_or_default(),
            std::env::var("NEWSDATA_API_KEY").unwrap_or_default(),
            NewsServiceSettings::new(APP_ID),
        );

        let history_service = HistoryService::new().expect("Failed to init DB");
        let history_worker = Arc::new(HistoryWorker::builder().launch(history_service).detach());

        let image_cache = ImageCache::new();

        let initial_section = NewsSection::General;
        let initial_key = initial_section.to_key();

        let category_page = CategoryPage::builder()
            .launch((
                initial_section,
                fetch_service.clone(),
                history_worker.clone(),
                image_cache.clone(),
                false,
            ))
            .forward(sender.input_sender(), |msg| match msg {
                PageOutput::ToggleSidebar => Msg::ToggleSidebar,
            });

        let mut pages_cache = HashMap::new();
        pages_cache.insert(initial_key.clone(), PageController::Category(category_page));

        let sidebar =
            SideBar::builder()
                .launch(())
                .forward(sender.input_sender(), move |message| match message {
                    SidebarMessage::ToggleSidebar => Msg::ToggleSidebar,
                    SidebarMessage::SelectSection(section) => Msg::ChangeSection(section),
                    SidebarMessage::SelectPage(page) => Msg::ChangeSectionPage(page),
                });

        let model = App {
            sidebar_visible: true,
            fetch_service,
            image_cache,
            pages_cache,
            current_page_key: initial_key,
            sidebar,
            history_worker,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Msg::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                if let Some(PageController::Category(c)) = self.pages_cache.get(&self.current_page_key) {
                    let _ = c.sender().send(PageInput::ShowSidebarToggleBtn(!self.sidebar_visible));
                }
            }

            Msg::ChangeSection(section) => {
                let key = section.to_key();

                if !self.pages_cache.contains_key(&key) {
                    let new_page = CategoryPage::builder()
                        .launch((
                            section,
                            self.fetch_service.clone(),
                            self.history_worker.clone(),
                            self.image_cache.clone(),
                            !self.sidebar_visible,
                        ))
                        .forward(sender.input_sender(), |msg| match msg {
                            PageOutput::ToggleSidebar => Msg::ToggleSidebar,
                        });

                    self.pages_cache.insert(key.clone(), PageController::Category(new_page));
                }

                self.current_page_key = key;
            }

            Msg::ChangeSectionPage(page) => {
                let key = page.to_key();
                
                if !self.pages_cache.contains_key(&key) {
                    match page {
                        NavigationPage::History => {
                            let history_page = HistoryPage::builder()
                                .launch((self.history_worker.clone(), !self.sidebar_visible))
                                .forward(sender.input_sender(), |msg| match msg {
                                    HistoryPagePageOutput::ToggleSidebar => Msg::ToggleSidebar,
                                });
                            
                            self.pages_cache.insert(key.clone(), PageController::History(history_page));
                        }
                        _ => {}
                    }
                }

                self.current_page_key = key;
            }
        }
    }
}

fn main() {
    dotenv().ok();

    if let Ok(path) = std::fs::canonicalize("./data") {
        let path_str = path.to_string_lossy();
        let clean_path = path_str.trim_start_matches(r"\\?\").to_string();
        unsafe {
            std::env::set_var("GSETTINGS_SCHEMA_DIR", clean_path);
        }
    }

    let app = RelmApp::new(APP_ID);
    gtk::gio::resources_register_include!("frostnews.gresource").expect("Resources failed");
    relm4::set_global_css_from_file("resources/style/style.css").expect("CSS failed");

    app.run::<App>(());
}