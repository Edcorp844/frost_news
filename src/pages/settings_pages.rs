use adw::NavigationPage;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, CompositeTemplate};

use crate::types::cache::ImageCache;

mod imp {
    use crate::types::cache::ImageCache;

    use super::*;
    use adw::subclass::prelude::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/example/frostnews/ui/category_pages/setting_page.ui")]
    pub struct HealthNewsPage {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub open_sidebar_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub root_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub main_content: TemplateChild<gtk::Box>,
        pub cache: std::cell::OnceCell<ImageCache>,
        pub root_split_view: std::cell::OnceCell<adw::NavigationSplitView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HealthNewsPage {
        const NAME: &'static str = "HealthNewsPage";
        type Type = super::HealthNewsPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthNewsPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for HealthNewsPage {}
    impl NavigationPageImpl for HealthNewsPage {}
}

glib::wrapper! {
    pub struct HealthNewsPage(ObjectSubclass<imp::HealthNewsPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl HealthNewsPage {
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
