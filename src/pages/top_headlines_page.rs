use std::sync::Arc;

use adw::NavigationPage;
use chrono::Local;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, CompositeTemplate};

use crate::pages::news_page::NewsPage;
use crate::types::cache::ImageCache;
use crate::types::news_article::NewsArticle;
use crate::ui_build_herlper_functions::ui_content_organiser::time_organiser::UITimeOrganiser;
use crate::widgets::grid_news_tile::NewsRowTile;
use crate::widgets::headline_news_tile::HeadlineNewsTile;

mod imp {
    use super::*;
    use adw::subclass::prelude::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/example/frostnews/ui/category_pages/topheadlines.ui")]
    pub struct TopHeadlinesPage {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub open_sidebar_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub root_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub title_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub current_date_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub featured_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub articles_container: TemplateChild<gtk::Box>,
        pub cache: std::cell::OnceCell<ImageCache>,
        pub root_split_view: std::cell::OnceCell<adw::NavigationSplitView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TopHeadlinesPage {
        const NAME: &'static str = "TopHeadlinesPage";
        type Type = super::TopHeadlinesPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TopHeadlinesPage {
        fn constructed(&self) {
            self.parent_constructed();
            let date = Local::now().format("%A, %B %d, %Y").to_string();
            self.current_date_label.set_text(&date);
        }
    }

    impl WidgetImpl for TopHeadlinesPage {}
    impl NavigationPageImpl for TopHeadlinesPage {}
}

glib::wrapper! {
    pub struct TopHeadlinesPage(ObjectSubclass<imp::TopHeadlinesPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TopHeadlinesPage {
    pub fn new(cache: ImageCache, parent: &adw::NavigationSplitView) -> Self {
        let page: Self = glib::Object::new();

        if page.imp().cache.set(cache).is_err() {
            eprintln!("Warning: Cache was already initialized");
        }
        if page.imp().root_split_view.set(parent.clone()).is_err() {
            eprintln!("Warning: Root Split View was already initialized");
        }
        page.bind_toggle_side_bar();

        parent.set_content(Some(&page));

        page
    }

    pub fn clear(&self) {
        let imp = self.imp();

        while let Some(child) = imp.featured_box.first_child() {
            imp.featured_box.remove(&child);
        }

        while let Some(child) = imp.articles_container.first_child() {
            imp.articles_container.remove(&child);
        }
    }

    pub fn cache(&self) -> ImageCache {
        self.imp()
            .cache
            .get()
            .expect("Cache not initialized")
            .clone()
    }

    pub fn root(&self) -> adw::NavigationSplitView {
        self.imp()
            .root_split_view
            .get()
            .expect("Root Split Viw Not initialized")
            .clone()
    }

    pub fn show_loading(&self) {
        let imp = self.imp();
        imp.root_stack.set_visible_child_name("loading");
    }

    pub fn show_content(&self) {
        let imp = self.imp();
        imp.root_stack.set_visible_child_name("content");
    }

    pub fn add_top_headlines(
        &self,
        articles: &[Arc<dyn NewsArticle>],
    ) {
        let imp = self.imp();
        let header_box = gtk::Box::builder()
            .spacing(10)
            .margin_top(24)
            .margin_bottom(30)
            .margin_start(12)
            .css_classes(vec!["clickable-header"])
            .build();

        let chevron = gtk::Image::builder()
            .icon_name("pan-down-symbolic")
            .pixel_size(25)
            .css_classes(vec!["header-chevron"])
            .build();

        let label = gtk::Label::builder()
            .label("TOP STORIES")
            .xalign(0.0)
            .css_classes(vec!["caption-heading"])
            .build();

        header_box.append(&label);
        header_box.append(&chevron);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(16)
            .margin_bottom(30)
            .margin_top(12)
            .build();

        for (idx, article) in articles.iter().enumerate() {
            let page = self.clone();
            let parent_clone = self.root();
            let article_clone = article.clone();

            // --- Related Articles Logic ---
            let mut related_articles = Vec::new();
            let total = articles.len();

            // Pick 5 articles starting from the one after the current index
            for i in 1..=5 {
                if total <= 1 {
                    break;
                } // Handle edge case with only 1 article

                let next_idx = (idx + i) % total;

                // Don't add the current article to its own related list
                if next_idx != idx {
                    related_articles.push(articles[next_idx].clone());
                }
            }

            let image_cache_clone1 = self.cache().clone();
            let image_cache_clone2 = self.cache().clone();
            let tile = HeadlineNewsTile::widget(
                article.title().as_str(),
                article.url_to_image().clone(),
                article.published_at().clone(),
                article.description().clone(),
                image_cache_clone1,
                move || {
                    let news_page = NewsPage::new(
                        article_clone.clone(),
                        related_articles.clone(),
                        image_cache_clone2.clone(),
                    );

                    news_page.bind_toggle_side_bar(&parent_clone);
                    page.push_page(&news_page);
                },
            );

            container.append(&tile);
        }

        let revealer = gtk::Revealer::builder()
            .child(&container)
            .transition_type(gtk::RevealerTransitionType::SlideDown)
            .reveal_child(true) // Start expanded
            .build();

        let header_gesture = gtk::GestureClick::new();
        header_gesture.connect_released(glib::clone!(
            #[weak]
            revealer,
            #[weak]
            chevron,
            move |_, _, _, _| {
                let is_now_revealing = !revealer.reveals_child();
                revealer.set_reveal_child(is_now_revealing);

                if is_now_revealing {
                    chevron.set_icon_name(Some("pan-down-symbolic"));
                } else {
                    chevron.set_icon_name(Some("pan-end-symbolic"));
                }
            }
        ));
        header_box.add_controller(header_gesture);
        imp.featured_box.append(&header_box);
        imp.featured_box.append(&revealer);
    }

    pub fn add_articles_sections<T: NewsArticle + Clone + 'static>(
        &self,
        grouped: &std::collections::BTreeMap<chrono::DateTime<chrono::Utc>, Vec<T>>,
    ) {
        let imp = self.imp();
        let time_organiser = UITimeOrganiser::new();

        let mut buckets: Vec<_> = grouped.keys().cloned().collect();
        buckets.sort_by(|a, b| b.cmp(a));

        let buckest_clone = buckets.clone();

        for bucket in buckets {
            let label_text = time_organiser.categorize_by_relative_time(bucket);

            let header_box = gtk::Box::builder()
                .spacing(10)
                .margin_top(24)
                .margin_bottom(12)
                .margin_start(12)
                .css_classes(vec!["clickable-header"])
                .build();

            let chevron = gtk::Image::builder()
                .icon_name("pan-down-symbolic")
                .pixel_size(25)
                .css_classes(vec!["header-chevron"])
                .build();

            let label = gtk::Label::builder()
                .label(&label_text.to_uppercase())
                .xalign(0.0)
                .css_classes(vec!["caption-heading"])
                .build();

            header_box.append(&label);
            header_box.append(&chevron);

            let grid = gtk::FlowBox::builder()
                .selection_mode(gtk::SelectionMode::None)
                .row_spacing(20)
                .column_spacing(10)
                .build();

            let last_width = std::cell::Cell::new(0);
            grid.add_tick_callback(move |grid, _| {
                let width = grid.width();
                if width != last_width.get() {
                    last_width.set(width);
                    let columns = match width {
                        0..=500 => 2,
                        501..=800 => 3,
                        801..=1100 => 4,
                        _ => 5,
                    };
                    grid.set_min_children_per_line(columns);
                    grid.set_max_children_per_line(columns);
                }
                glib::ControlFlow::Continue
            });

            let revealer = gtk::Revealer::builder()
                .child(&grid)
                .transition_type(gtk::RevealerTransitionType::SlideDown)
                .reveal_child(true) // Start expanded
                .build();

            let header_gesture = gtk::GestureClick::new();
            header_gesture.connect_released(glib::clone!(
                #[weak]
                revealer,
                #[weak]
                chevron,
                move |_, _, _, _| {
                    let is_now_revealing = !revealer.reveals_child();
                    revealer.set_reveal_child(is_now_revealing);

                    if is_now_revealing {
                        chevron.set_icon_name(Some("pan-down-symbolic"));
                    } else {
                        chevron.set_icon_name(Some("pan-end-symbolic"));
                    }
                }
            ));
            header_box.add_controller(header_gesture);
           
            if let Some(articles) = grouped.get(&bucket) {
                let all_articles: Vec<T> = buckest_clone
                    .iter()
                    .filter_map(|b| grouped.get(b))
                    .flatten()
                    .cloned()
                    .collect();

                for (_, article) in articles.iter().enumerate() {
                    let article_clone = article.clone();
                    let page_clone = self.clone();
                    let parent_clone = self.root();

                    let mut related_articles = Vec::new();
                    let total = all_articles.len();

                    // Find the index of the current article in the flattened list
                    // (We calculate this based on bucket position + current loop index)
                    let global_idx = all_articles
                        .iter()
                        .position(|a| a.title() == article.title())
                        .unwrap_or(0);

                    for i in 1..=5 {
                        if total <= 1 {
                            break;
                        } // Only one article exists

                        // This logic wraps around: (current + offset) % total
                        let next_idx = (global_idx + i) % total;

                        // Ensure we don't add the current article to its own related list
                        if next_idx != global_idx {
                            related_articles.push(all_articles[next_idx].clone());
                        }
                    }

                    let image_cache_clone1 = self.cache().clone();
                    let image_cache_clone2 = self.cache().clone();
                    let tile = NewsRowTile::widget(
                        &article.title(),
                        article.description(),
                        article.url_to_image(),
                        article.published_at(),
                        image_cache_clone1,
                        move || {
                            let news_page = NewsPage::new(
                                article_clone.clone(),
                                related_articles.clone(),
                                image_cache_clone2.clone(),
                            );

                            news_page.bind_toggle_side_bar(&parent_clone);
                            page_clone.push_page(&news_page);
                        },
                    );
                    grid.append(&tile);
                }
            }

            imp.articles_container.append(&header_box);
            imp.articles_container.append(&revealer);
        }
    }

    pub fn set_title(&self, title: &str) {
        self.imp().title_label.set_text(title);
    }

    pub fn set_date(&self, date: &str) {
        self.imp().current_date_label.set_text(date);
    }

    pub fn set_date_from_format(&self, format: &str) {
        let date = Local::now().format(format).to_string();
        self.imp().current_date_label.set_text(&date);
    }

    pub fn bind_toggle_side_bar(&self) {
        let imp = self.imp();
        let btn = imp.open_sidebar_btn.get();
        let split_view = self.root();

        btn.connect_clicked(glib::clone!(
            #[weak]
            split_view,
            move |_| {
                split_view.set_collapsed(false);
            }
        ));

        split_view
            .bind_property("collapsed", &btn, "visible")
            .sync_create()
            .build();
    }

    pub fn push_page(&self, page: &impl IsA<NavigationPage>) {
        let imp = self.imp();
        imp.navigation_view.push(page);
    }
}
