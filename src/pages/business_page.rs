use crate::NewsFetchService;
use crate::components::categorised_news::CategorisedNewsSection;
use crate::pages::news_page::{NewsPage, NewsPageInput, NewsPageOutput};
use crate::types::cache::ImageCache;
use crate::types::news_article::NewsArticle;
use crate::types::news_category::NewsSection;
use crate::types::news_handler::NewsHandler;
use crate::types::news_source::NewsSource;
use crate::utils::time_organizer;

use adw::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct BusinessPage {
    show_sidebar_toggle_btn: bool,
    sections: FactoryVecDeque<CategorisedNewsSection>,
    fetch_service: NewsFetchService,
    cache: ImageCache,
    news_page_controller: Option<Controller<NewsPage>>,
    navigation_view: adw::NavigationView,
    category: NewsSection,
}

#[derive(Debug)]
pub enum BusinessPageInput {
    FetchNews,
    NewsReceived(BTreeMap<String, Vec<Arc<dyn NewsArticle>>>),
    GotoNews((Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>)),
    ApiError(String),
    ShowSidebarToggleBtn(bool),
}

impl NewsHandler for BusinessPageInput {
    fn on_news_received(grouped: BTreeMap<String, Vec<Arc<dyn NewsArticle>>>) -> Self {
        BusinessPageInput::NewsReceived(grouped)
    }
    fn on_error(err: String) -> Self {
        BusinessPageInput::ApiError(err)
    }
}

#[derive(Debug)]
pub enum BusinessPageOutput {
    ToggleSidebar,
}

#[relm4::component(pub)]
impl Component for BusinessPage {
    type Init = (NewsSection, NewsFetchService, ImageCache, bool);
    type Input = BusinessPageInput;
    type Output = BusinessPageOutput;
    type CommandOutput = ();

    view! {


            adw::NavigationPage{
                 #[wrap(Some)]
                 set_child = &model.navigation_view.clone() {

                 push = &adw::NavigationPage{
                    #[wrap(Some)]
                        set_child = &adw::ToolbarView {
                            add_top_bar = &adw::HeaderBar {
                                set_margin_start: 20,
                                set_show_title: false,
                                pack_start = &gtk::Button {
                                    set_icon_name: "sidebar-show-symbolic",
                                    set_tooltip: "Show Sidebar",
                                    #[watch]
                                    set_visible: model.show_sidebar_toggle_btn,
                                    connect_clicked[sender] =>move |_|{
                                        sender.output(BusinessPageOutput::ToggleSidebar).unwrap()
                                    } ,
                                },
                            },
                            #[wrap(Some)]
                    set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,

                    adw::Clamp {
                        set_maximum_size: 1400,
                        set_tightening_threshold: 1000,


                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 24,
                            set_margin_all: 20,

                            gtk::Label {
                                set_label: "FROST NEWS",
                                add_css_class: "accent",
                                add_css_class: "frost-brand-title",
                                set_xalign: 0.0,
                            },

                            gtk::Label {
                                set_label: chrono::Local::now().format("%A, %B %d, %Y").to_string().as_str(),
                                add_css_class: "frost-date-subtitle",
                                add_css_class: "dim-label",
                                set_xalign: 0.0,
                            },

                            #[local_ref]
                            sections-> gtk::Box{

                            }
                        }
                    }
                }
            }
        }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (category, fetch_service, cache, show_sidebar_toggle_btn) = init;

        let sections = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 12))
            .forward(sender.input_sender(), move |data| BusinessPageInput::GotoNews(data));

        let navigation_view = adw::NavigationView::builder().build();

        let model = BusinessPage {
            show_sidebar_toggle_btn,
            sections,
            fetch_service,
            cache,
            navigation_view,
            news_page_controller: None,
            category
        };

        let sections = model.sections.widget();

        let widgets = view_output!();

        sender.input(BusinessPageInput::FetchNews);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            BusinessPageInput::ShowSidebarToggleBtn(visible) => {
                self.show_sidebar_toggle_btn = visible;
                if self.news_page_controller.is_some() {
                    let _ = self.news_page_controller.as_ref().unwrap().sender().send(
                        NewsPageInput::ShowSidebarToggleBtn(self.show_sidebar_toggle_btn),
                    );
                }
            }
            BusinessPageInput::FetchNews => {
                let source = NewsSource::NewsAPI;
                let sender_clone = sender.clone();
                self.fetch_service
                    .fetch_news(self.category.clone(),0, sender_clone);
            }
            BusinessPageInput::NewsReceived(grouped_data) => {
                let mut guard = self.sections.guard();
                guard.clear();
                let mut buckets: Vec<_> = grouped_data.keys().cloned().collect();
                buckets
                    .sort_by(|a, b| time_organizer::UITimeOrganiser::comapre(a.clone(), b.clone()));

                for bucket in buckets {
                    if let Some(articles) = grouped_data.get(&bucket) {
                        guard.push_back((bucket, articles.to_vec(), self.cache.clone()));
                    }
                }
            }
            BusinessPageInput::GotoNews(data) => {
                let (article, related_articles) = data;

                self.news_page_controller = Some(
                    NewsPage::builder()
                        .launch((
                            article.clone(),
                            related_articles.clone(),
                            self.cache.clone(),
                            self.show_sidebar_toggle_btn,
                        ))
                        .forward(sender.output_sender(), move |action| match action {
                            NewsPageOutput::ToggleSidebar => BusinessPageOutput::ToggleSidebar,
                        }),
                );

                if self.news_page_controller.is_some() {
                    self.navigation_view
                        .push(self.news_page_controller.as_ref().unwrap().widget());
                }
            }
            BusinessPageInput::ApiError(e) => {
                eprintln!("Error loading news: {}", e);
            }
        }
    }
}
