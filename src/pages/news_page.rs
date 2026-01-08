use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, CompositeTemplate};

use crate::types::cache::ImageCache;
use crate::types::news_article::NewsArticle;
use crate::ui_build_herlper_functions::image_functions::image_loader;
use crate::widgets::grid_news_tile::NewsRowTile;

mod imp {
    use std::cell::RefCell;

    use crate::types::news_article::NewsArticle;

    use super::*;
    use adw::subclass::prelude::NavigationPageImpl;
    use gtk::subclass::prelude::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/example/frostnews/ui/newspage.ui")]
    pub struct NewsPage {
        #[template_child]
        pub open_sidebar_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub root_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub page_header: TemplateChild<gtk::Box>,
        #[template_child]
        pub article_section: TemplateChild<gtk::Box>,
        #[template_child]
        pub related_articles: TemplateChild<gtk::Box>,
        pub cache: std::cell::OnceCell<ImageCache>,
        pub related: RefCell<Vec<Box<dyn NewsArticle>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NewsPage {
        const NAME: &'static str = "NewsPage";
        type Type = super::NewsPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NewsPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for NewsPage {}
    impl NavigationPageImpl for NewsPage {}
}

glib::wrapper! {
    pub struct NewsPage(ObjectSubclass<imp::NewsPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NewsPage {
    pub fn new<T: NewsArticle + Clone + 'static>(
        article: T,
        related_articles: Vec<T>,
        cache: ImageCache,
    ) -> Self {
        let page: Self = glib::Object::new();

        if page.imp().cache.set(cache).is_err() {
            eprintln!("Warning: Cache was already initialized");
        }

        page.set_article(article);
        page.set_related_articles(related_articles);

        page
    }

    pub fn cache(&self) -> ImageCache {
        self.imp()
            .cache
            .get()
            .expect("Cache not initialized")
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

    pub fn set_article<T: NewsArticle>(&self, article: T) {
        self.show_loading();
        let imp = self.imp();

        let row_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12) // Space between image and text
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .css_classes(vec!["news-grid-tile"])
            .build();

        let image_frame = gtk::Frame::builder()
            .width_request(400)
            .height_request(300)
            .hexpand(false)
            .vexpand(false)
            .css_classes(vec!["news-tile-image-frame"])
            .build();

        let image = gtk::Picture::builder()
            .content_fit(gtk::ContentFit::Cover)
            .css_classes(vec!["news-tile-image"])
            .build();

        image_frame.set_child(Some(&image));

        let news_content_frame = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .build();

        let title = gtk::Label::builder()
            .label(article.title())
            .wrap(true)
            .xalign(0.0)
            .margin_start(16)
            .margin_bottom(16)
            .margin_end(16)
            .margin_top(16)
            .css_classes(vec!["title-1", "news-page-article-tile"])
            .build();

        let news_description = gtk::Label::builder()
            .label(article.description().unwrap_or_default())
            .wrap(true)
            .max_width_chars(50)
            .wrap_mode(pango::WrapMode::WordChar)
            .xalign(0.0)
            .margin_start(16)
            .margin_bottom(16)
            .margin_end(16)
            .css_classes(vec!["document", "dimmed", "numeric"])
            .build();

        let news_content = gtk::Label::builder()
            .label(article.content().unwrap_or_default())
            .wrap(true)
            .max_width_chars(50)
            .wrap_mode(pango::WrapMode::WordChar)
            .xalign(0.0)
            .margin_start(16)
            .margin_bottom(16)
            .margin_end(16)
            .css_classes(vec!["document", "dimmed", "numeric"])
            .build();

        let open_url_btn = gtk::Button::builder()
            .label("Open Link")
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .margin_start(16)
            .margin_bottom(16)
            .margin_end(16)
            .margin_top(16)
            .css_classes(vec!["suggested-action", "raised", "pill"])
            .build();

        news_content_frame.append(&title);
        news_content_frame.append(&news_description);
        news_content_frame.append(&news_content);
        news_content_frame.append(&open_url_btn);

        row_container.append(&image_frame); // First child = Left
        row_container.append(&news_content_frame); // Second child = Right

        imp.article_section.append(&row_container);
        self.show_content();
        let image_loader = image_loader::ImageLoader::new();
        let cache = self.cache();
        image_loader.load_picture_image(&image, article.url_to_image(), cache);
    }

    pub fn set_related_articles<T: NewsArticle + Clone + 'static>(&self, related_articles: Vec<T>) {
        let imp = self.imp();

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
            .label("RELATED")
            .xalign(0.0)
            .css_classes(vec!["caption-heading"])
            .build();

        header_box.append(&label);
        header_box.append(&chevron);

        // --- THE GRID (CONTENT) ---
        let grid = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .row_spacing(20)
            .column_spacing(10)
            .build();

        // Responsive columns logic
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

        // --- THE REVEALER (ANIMATION) ---
        let revealer = gtk::Revealer::builder()
            .child(&grid)
            .transition_type(gtk::RevealerTransitionType::SlideDown)
            .reveal_child(true) // Start expanded
            .build();

        // --- TOGGLE CLICK LOGIC ---
        let header_gesture = gtk::GestureClick::new();
        header_gesture.connect_released(glib::clone!(
            #[weak]
            revealer,
            #[weak]
            chevron,
            move |_, _, _, _| {
                let is_now_revealing = !revealer.reveals_child();
                revealer.set_reveal_child(is_now_revealing);

                // Update icon based on state
                if is_now_revealing {
                    chevron.set_icon_name(Some("pan-down-symbolic"));
                } else {
                    chevron.set_icon_name(Some("pan-end-symbolic"));
                }
            }
        ));
        header_box.add_controller(header_gesture);

        // --- POPULATE ARTICLES ---

        // Inside set_related_articles<T>(...)
        for (idx, article) in related_articles.iter().enumerate() {
            let article_clone = article.clone();
            let page_clone = self.clone();
            let cache = self.cache();

            // We need a clone of the full list to calculate the NEXT set of 5
            let full_list_clone = related_articles.clone();

            let tile = NewsRowTile::widget(
                &article.title(),
                article.description(),
                article.url_to_image(),
                article.published_at(),
                cache,
                move || {
                    // 1. Calculate the NEW related articles for the clicked article
                    let mut next_related = Vec::new();
                    let total = full_list_clone.len();

                    for i in 1..=5 {
                        if total <= 1 {
                            break;
                        }
                        let next_idx = (idx + i) % total;
                        if next_idx != idx {
                            next_related.push(full_list_clone[next_idx].clone());
                        }
                    }

                    // 2. Refresh the current page with the new content
                    page_clone.clear();
                    page_clone.set_article(article_clone.clone());
                    page_clone.set_related_articles(next_related);

                    // 3. Optional: Scroll back to the top so the user sees the new article
                    // imp.scrolled_window.get_vadjustment().set_value(0.0);
                },
            );
            grid.append(&tile);
        }

        // Add header and the animated container to the page
        imp.related_articles.append(&header_box);
        imp.related_articles.append(&revealer);
    }

    pub fn clear(&self) {
        let imp = self.imp();
        while let Some(child) = imp.related_articles.first_child() {
            imp.related_articles.remove(&child);
        }
        while let Some(child) = imp.article_section.first_child() {
            imp.article_section.remove(&child);
        }
    }

    pub fn bind_toggle_side_bar(&self, split_view: &adw::NavigationSplitView) {
        let imp = self.imp();
        let btn = imp.open_sidebar_btn.get();

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
}
