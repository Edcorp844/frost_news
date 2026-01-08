use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

use crate::{
    services::news_service::fetch_service::NewsFetchService,
    types::{app_config::AppConfig, news_category::NewsCategory, news_source::NewsSource},
    ui_build_herlper_functions::constants::news_categories::CATEGORIES,
    widgets::sidebar::SideBar,
};

glib::wrapper! {
    pub struct FrostNewsAPPWindow(ObjectSubclass<imp::FrostNewsAPPWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

mod imp {
    use crate::{
        services::news_service::fetch_service::NewsFetchService, widgets::sidebar::SideBar,
    };

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/example/frostnews/ui/frostnews.ui")]
    pub struct FrostNewsAPPWindow {
        #[template_child]
        pub split_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        pub toggle_sidebar_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub sidebar_toolbar: TemplateChild<adw::ToolbarView>,
        pub news_service: std::cell::OnceCell<NewsFetchService>,
        pub side_bar: std::cell::OnceCell<SideBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FrostNewsAPPWindow {
        const NAME: &'static str = "FrostNewsAPPWindow";
        type Type = super::FrostNewsAPPWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FrostNewsAPPWindow {
        fn constructed(&self) {
            self.parent_constructed();
            if self
                .side_bar
                .set(SideBar::new(&self.sidebar_toolbar))
                .is_err()
            {
                eprintln!("Warning: Window couldn't Set SideBar");
            }
        }
    }

    impl WidgetImpl for FrostNewsAPPWindow {}
    impl WindowImpl for FrostNewsAPPWindow {}
    impl ApplicationWindowImpl for FrostNewsAPPWindow {}
    impl AdwApplicationWindowImpl for FrostNewsAPPWindow {}
}

impl FrostNewsAPPWindow {
    pub fn new() -> Self {
        let window: Self = glib::Object::new();
        window.set_sidebar_toggle_btn();
        match window.get_sidebar().build() {
            Ok(()) => {}
            Err(e) => println!("Failed to create Side bar: {}", e),
        }
        window
    }

    pub fn config(&self, config: AppConfig) {
        let imp = self.imp();
        let _ = imp.news_service.set(config.news_fetch_service);
    }

    pub fn initialise(&self) {
        self.bind_sidebar_children();
        self.get_news_service()
            .fetch_news(NewsCategory::General, NewsSource::NewsAPI);
        self.get_sidebar().select_first_category();
    }

    pub fn get_split_view(&self) -> adw::NavigationSplitView {
        self.imp().split_view.clone()
    }

    pub fn get_sidebar(&self) -> SideBar {
        self.imp()
            .side_bar
            .get()
            .expect("SideBar not initialized")
            .clone()
    }

    fn bind_sidebar_children(&self) {
        let side_bar = self.get_sidebar();
        let service = self.get_news_service();

        // Use glib::clone to move the service into the sidebar's event handler
        side_bar.connect_sidebar(glib::clone!(
            #[strong]
            service,
            move |row| {
                // 1. Find the category enum based on the row's ID
                let matched_category = CATEGORIES
                    .iter()
                    .find(|c| c.id == row)
                    .map(|c| c.enum_name.clone());

                // 2. Execute the fetch based on the match
                if let Some(category_enum) = matched_category {
                    println!("Matched to category: {:?}", row);
                    service.fetch_news(category_enum, NewsSource::NewsAPI);
                } else {
                    // Check if it's the expander/header (which usually has no ID)
                    if !row.is_empty() {
                        println!("No category found for ID: {}, defaulting to General", row);
                        service.fetch_news(NewsCategory::General, NewsSource::NewsAPI);
                    }
                }
            }
        ));
    }

    pub fn get_news_service(&self) -> NewsFetchService {
        self.imp()
            .news_service
            .get()
            .expect("NEWSDATA API KEY  not initialized")
            .clone()
    }

    fn set_sidebar_toggle_btn(&self) {
        let splitview_clone = self.get_split_view();
        self.imp().toggle_sidebar_btn.connect_clicked(glib::clone!(
            #[weak]
            splitview_clone,
            move |_| {
                let is_collapsed = splitview_clone.is_collapsed();
                splitview_clone.set_collapsed(!is_collapsed);
                splitview_clone.set_show_content(!is_collapsed);
            }
        ));
    }
}
