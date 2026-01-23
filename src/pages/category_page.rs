use crate::NewsFetchService;
use crate::components::categorised_news::CategorisedNewsSection;
use crate::pages::news_page::{NewsPage, NewsPageInput, NewsPageOutput};
use crate::services::workers::history_worker::{HistoryWorker, HistoryWorkerInput};
use crate::types::cache::ImageCache;
use crate::types::news_article::NewsArticle;
use crate::types::news_category::NewsSection;
use crate::types::news_handler::NewsHandler;
use crate::types::persistent_articel::PersistentArticle;
use crate::utils::page_pignation::NewsPagination;
use crate::utils::time_organizer;

use adw::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct CategoryPage {
    show_sidebar_toggle_btn: bool,
    sections: FactoryVecDeque<CategorisedNewsSection>,
    fetch_service: NewsFetchService,
    cache: ImageCache,
    news_page_controller: Option<Controller<NewsPage>>,
    navigation_view: adw::NavigationView,
    category: NewsSection,
    is_refreshing: bool,
    error_message: Option<String>,
    is_loading: bool,
    pagination: NewsPagination,
    reached_end: bool,
    // we need this for history sync
    history_worker: Arc<Controller<HistoryWorker>>,
}

#[derive(Debug)]
pub enum PageInput {
    FetchNews,
    Refresh,
    NextPage,
    PreviousPage,
    CopyError,
    LoadPage(i32),
    NewsReceived(BTreeMap<String, Vec<Arc<dyn NewsArticle>>>),
    GotoNews((Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>)),
    ApiError(String),
    ShowSidebarToggleBtn(bool),
}

impl NewsHandler for PageInput {
    fn on_news_received(grouped: BTreeMap<String, Vec<Arc<dyn NewsArticle>>>) -> Self {
        PageInput::NewsReceived(grouped)
    }
    fn on_error(err: String) -> Self {
        PageInput::ApiError(err)
    }
}

#[derive(Debug)]
pub enum PageOutput {
    ToggleSidebar,
}

#[relm4::component(pub)]
impl Component for CategoryPage {
    type Init = (
        NewsSection,
        NewsFetchService,
        Arc<Controller<HistoryWorker>>,
        ImageCache,
        bool,
    );
    type Input = PageInput;
    type Output = PageOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &model.navigation_view.clone() {
                push = &adw::NavigationPage {
                    #[wrap(Some)]
                    set_child = match model.is_loading {
                        // --- FIRST TIME LOADING STATE ---
                        true => {
                            adw::NavigationPage {
                                #[wrap(Some)]
                                set_child = &adw::ToolbarView {
                                    add_top_bar = &adw::HeaderBar {
                                        set_margin_start: 20,
                                        set_show_title: false,
                                        pack_start = &gtk::Button {
                                            set_icon_name: "sidebar-show-symbolic",
                                            #[watch]
                                            set_visible: model.show_sidebar_toggle_btn,
                                            connect_clicked[sender] => move |_| {
                                                sender.output(PageOutput::ToggleSidebar).unwrap()
                                            }
                                        },
                                    },
                                    #[wrap(Some)]
                                    set_content = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_halign: gtk::Align::Center,
                                        set_valign: gtk::Align::Center,
                                        set_margin_all: 20,

                                        match model.error_message.is_some() {
                                            false => adw::Spinner {
                                                add_css_class: "spin-glow",
                                                set_height_request: 50,
                                                set_width_request: 50,
                                            },

                                            true => gtk::Frame {
                                                add_css_class: "news-tile-image-frame",
                                                set_margin_bottom: 12,

                                                gtk::Box{
                                                    set_orientation: gtk::Orientation::Vertical,
                                                    set_halign: gtk::Align::Center,
                                                    set_valign: gtk::Align::Center,
                                                    set_margin_all: 50,
                                                    set_spacing: 40,

                                                    gtk::Box {
                                                        set_halign: gtk::Align::Center,
                                                        set_valign: gtk::Align::Center,
                                                        set_spacing: 10,
                                                        add_css_class: "error-title-box",

                                                        gtk::Image{
                                                            set_icon_name: Some("dialog-error-symbolic"),
                                                        },

                                                        gtk::Label{
                                                            set_label: "Error",
                                                        }
                                                    },

                                                    gtk::Label{
                                                        #[watch]
                                                        set_label: model.error_message.as_deref().unwrap_or(""),
                                                        set_wrap: true,
                                                        set_halign: gtk::Align::Center,
                                                        set_justify: gtk::Justification::Center,
                                                        set_margin_bottom: 6,
                                                        set_margin_horizontal: 20,
                                                        add_css_class: "monospace",
                                                        add_css_class: "error",
                                                    },

                                                     gtk::Box {
                                                        set_orientation: gtk::Orientation::Horizontal,
                                                        set_halign: gtk::Align::Center,
                                                        set_spacing: 12,
                                                        set_margin_bottom: 20,

                                                        gtk::Button {
                                                            set_icon_name: "go-previous-symbolic",
                                                            add_css_class: "circular",
                                                            #[watch]
                                                            set_sensitive: model.pagination.current_page > 1,
                                                            connect_clicked[sender] => move |_| {
                                                                sender.input(PageInput::PreviousPage);
                                                            }
                                                        },

                                                        gtk::Button {
                                                            set_icon_name: "edit-copy-symbolic",
                                                            add_css_class: "circular",
                                                            set_tooltip_text: Some("Copy"),
                                                            connect_clicked[sender] => move |_| {
                                                                sender.input(PageInput::CopyError);
                                                            }
                                                        },

                                                        gtk::Button {
                                                            set_icon_name: "view-refresh-symbolic",
                                                            add_css_class: "circular",
                                                            set_tooltip_text: Some("Retry"),
                                                            connect_clicked[sender] => move |_| {
                                                                sender.input(PageInput::FetchNews);
                                                            }
                                                        }
                                                    }
                                                },
                                            }
                                    }
                                    }
                                }
                            }
                        }
                        // --- MAIN CONTENT STATE ---
                        false => {
                            #[name = "toast_overlay"]
                            adw::ToastOverlay {

                                #[wrap(Some)]
                                set_child = &adw::ToolbarView {
                                    add_top_bar = &adw::HeaderBar {
                                        set_margin_start: 20,
                                        add_css_class: "toolbar",
                                        set_show_title: false,
                                        pack_start = &gtk::Button {
                                            set_icon_name: "sidebar-show-symbolic",
                                            #[watch]
                                            set_visible: model.show_sidebar_toggle_btn,
                                            connect_clicked[sender] => move |_| {
                                                sender.output(PageOutput::ToggleSidebar).unwrap()
                                            }
                                        },
                                        pack_end = &gtk::Button {
                                            #[watch]
                                            set_icon_name: if model.is_refreshing {
                                                "process-working-symbolic"
                                            } else {
                                                "view-refresh-symbolic"
                                            },
                                            set_sensitive: !model.is_refreshing,
                                            connect_clicked[sender] => move |_| {
                                                sender.input(PageInput::Refresh)
                                            }
                                        }
                                    },

                                    #[wrap(Some)]
                                    set_content = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        adw::Banner {
                                            set_title: "Updating News Feed...",
                                            #[watch]
                                            set_revealed: model.is_refreshing,
                                        },

                                        gtk::Separator {
                                            add_css_class: "tahoe-shimmer-line",
                                            #[watch]
                                            set_visible: model.is_refreshing,
                                            set_halign: gtk::Align::Fill,
                                        },



                                    adw::Banner {
                                        #[watch]
                                        set_title: model.error_message.as_deref().unwrap_or(""),
                                        #[watch]
                                        set_revealed: model.error_message.is_some(),
                                        set_button_label: Some("Retry"),
                                        connect_button_clicked[sender] => move |_| {
                                            sender.input(PageInput::Refresh);
                                        }
                                    },

                                        gtk::ScrolledWindow {
                                            set_vexpand: true,
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
                                                        add_css_class: "frost-brand-title",
                                                        set_xalign: 0.0,
                                                    },

                                                    gtk::Label {
                                                        set_label: &chrono::Local::now().format("%A, %B %d, %Y").to_string(),
                                                        add_css_class: "dim-label",
                                                        set_xalign: 0.0,
                                                    },

                                                    #[local_ref]
                                                    sections -> gtk::Box {
                                                        set_orientation: gtk::Orientation::Vertical,
                                                        set_spacing: 12,
                                                    },

                                                    // --- PAGINATION FOOTER ---
                                                    gtk::Box {
                                                        set_orientation: gtk::Orientation::Horizontal,
                                                        set_halign: gtk::Align::Center,
                                                        set_spacing: 12,
                                                        set_margin_bottom: 20,

                                                        gtk::Button {
                                                            set_icon_name: "go-previous-symbolic",
                                                            add_css_class: "circular",
                                                            #[watch]
                                                            set_sensitive: model.pagination.current_page > 1,
                                                            connect_clicked[sender] => move |_| {
                                                                sender.input(PageInput::PreviousPage);
                                                            }
                                                        },

                                                        gtk::Label {
                                                            #[watch]
                                                            set_label: &format!("Page {}", model.pagination.current_page),
                                                            add_css_class: "dim-label",
                                                        },

                                                        gtk::Button {
                                                            set_icon_name: "go-next-symbolic",
                                                            add_css_class: "circular",
                                                            #[watch]
                                                            // PERMANENT DISABLE if end is reached
                                                            set_sensitive: !model.reached_end,
                                                            connect_clicked[sender] => move |_| {
                                                                sender.input(PageInput::NextPage);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
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
        let (category, fetch_service, history_worker, cache, show_sidebar_toggle_btn) = init;

        let sections = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 12))
            .forward(sender.input_sender(), move |data| PageInput::GotoNews(data));

        let navigation_view = adw::NavigationView::builder().build();

        let model = CategoryPage {
            show_sidebar_toggle_btn,
            sections,
            fetch_service,
            cache,
            navigation_view,
            news_page_controller: None,
            category,
            is_refreshing: false,
            error_message: None,
            is_loading: true,
            pagination: NewsPagination::new(),
            reached_end: false,
            history_worker,
        };

        let sections = model.sections.widget();

        // --- SETUP SHORTCUT ---
        let controller = gtk::ShortcutController::new();

        // Create the "Trigger" (Ctrl + R)
        let trigger = gtk::ShortcutTrigger::parse_string("<Control>r");

        // Create the "Action" (This calls your Refresh input)
        let action = gtk::CallbackAction::new({
            let sender = sender.clone();
            move |_, _| {
                sender.input(PageInput::Refresh);
                gtk::glib::Propagation::Stop // Stop event from bubbling up
            }
        });

        let shortcut = gtk::Shortcut::new(trigger, Some(action));
        controller.add_shortcut(shortcut);

        // Attach the controller to the root widget
        root.add_controller(controller);

        let widgets = view_output!();

        sender.input(PageInput::FetchNews);

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets, // <--- Now you have access!
        msg: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            PageInput::ShowSidebarToggleBtn(visible) => {
                self.show_sidebar_toggle_btn = visible;
                if self.news_page_controller.is_some() {
                    let _ = self.news_page_controller.as_ref().unwrap().sender().send(
                        NewsPageInput::ShowSidebarToggleBtn(self.show_sidebar_toggle_btn),
                    );
                }
            }
            PageInput::FetchNews => {
                self.error_message = None;
                let sender_clone = sender.clone();
                self.fetch_service
                    .fetch_news(self.category.clone(), 0, sender_clone);
            }
            PageInput::Refresh => {
                self.is_refreshing = true;
                self.error_message = None;
                self.pagination.reset();
                sender.input(PageInput::FetchNews);
            }
            PageInput::NewsReceived(grouped_data) => {
                self.is_refreshing = false; // Stop the banner spinner

                if grouped_data.is_empty() {
                    self.reached_end = true;

                    // Show the Toast
                    let toast = adw::Toast::new("Reached end of pages");
                    toast.set_timeout(5);
                    widgets.toast_overlay.add_toast(toast);

                    // Logic fix: if we tried to load a page that doesn't exist, go back
                    if self.pagination.current_page > 0 {
                        self.pagination.current_page -= 1;
                    }
                } else {
                    self.reached_end = false;

                    // 1. Update the cache/pagination data
                    self.pagination
                        .pages
                        .insert(self.pagination.current_page, grouped_data.clone());

                    // 2. Update the Factory (The actual UI elements)
                    let mut guard = self.sections.guard();
                    guard.clear();

                    let mut buckets: Vec<_> = grouped_data.keys().cloned().collect();
                    buckets.sort_by(|a, b| {
                        time_organizer::UITimeOrganiser::comapre(a.clone(), b.clone())
                    });

                    for bucket in buckets {
                        if let Some(articles) = grouped_data.get(&bucket) {
                            guard.push_back((bucket, articles.to_vec(), self.cache.clone()));
                        }
                    }
                }

                self.is_loading = false;
            }
            PageInput::GotoNews(data) => {
                let (article, related_articles) = data;

                let persistent_arcticle = PersistentArticle::auto_create(
                    article.title(),
                    article.url(),
                    article.description(),
                    article.content(),
                    article.published_at(),
                    article.url_to_image().unwrap_or("".to_string()),
                );

                self.history_worker
                    .emit(HistoryWorkerInput::Save(persistent_arcticle));
                self.news_page_controller = Some(
                    NewsPage::builder()
                        .launch((
                            article.clone(),
                            related_articles.clone(),
                            self.cache.clone(),
                            self.show_sidebar_toggle_btn,
                        ))
                        .forward(sender.output_sender(), move |action| match action {
                            NewsPageOutput::ToggleSidebar => PageOutput::ToggleSidebar,
                        }),
                );

                if self.news_page_controller.is_some() {
                    self.navigation_view
                        .push(self.news_page_controller.as_ref().unwrap().widget());
                }
            }
            PageInput::NextPage => {
                let next = self.pagination.current_page + 1;
                sender.input(PageInput::LoadPage(next));
            }
            PageInput::PreviousPage => {
                if self.pagination.current_page > 1 {
                    let prev = self.pagination.current_page - 1;
                    sender.input(PageInput::LoadPage(prev));
                }
            }
            PageInput::LoadPage(page_num) => {
                if let Some(existing_data) = self.pagination.pages.get(&page_num) {
                    // INSTANT LOAD: Page is already in memory
                    self.pagination.current_page = page_num;
                    sender.input(PageInput::NewsReceived(existing_data.clone()));
                } else {
                    // FETCH LOAD: Need to call the API
                    self.is_refreshing = true;
                    self.pagination.current_page = page_num;
                    // Pass the page number to your fetch service
                    self.fetch_service
                        .fetch_news(self.category.clone(), page_num, sender.clone());
                }
            }
            PageInput::ApiError(e) => {
                self.is_refreshing = false;
                self.error_message = Some(format!("{}", e));
                println!("eror: {}", self.error_message.clone().unwrap());
            }
            PageInput::CopyError => {
                gtk::gdk::Display::default().unwrap().clipboard().set_text(
                    self.error_message
                        .clone()
                        .unwrap_or("".to_string())
                        .as_str(),
                );
                let toast = adw::Toast::new("Text Copied");
                toast.set_timeout(5);
                widgets.toast_overlay.add_toast(toast);
            }
        }

        self.update_view(widgets, sender);
    }
}
