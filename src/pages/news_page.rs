use std::sync::Arc;

use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::{
    components::categorised_news::CategorisedNewsSection,
    types::{cache::ImageCache, news_article::NewsArticle},
};

#[derive(Debug)]
pub struct NewsPage {
    show_sidebar_toggle_btn: bool,
    article: Arc<dyn NewsArticle>,
    cache: ImageCache,
    related_section: FactoryVecDeque<CategorisedNewsSection>,
}

#[derive(Debug, Clone)]
pub enum NewsPageInput {
    ShowSidebarToggleBtn(bool),
}

#[derive(Debug, Clone)]
pub enum NewsPageOutput {
    ToggleSidebar,
}

#[relm4::component(pub)]
impl Component for NewsPage {
    type Init = (
        Arc<dyn NewsArticle>,
        Vec<Arc<dyn NewsArticle>>,
        ImageCache,
        bool,
    );
    type Input = NewsPageInput;
    type Output = NewsPageOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: "News Article",
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_show_title: false,
                    set_margin_start: 20,
                    pack_start = &gtk::Button {
                        set_icon_name: "sidebar-show-symbolic",
                        set_tooltip: "Show Sidebar",
                        #[watch]
                        set_visible: model.show_sidebar_toggle_btn,
                        connect_clicked[sender] => move |_| {
                            sender.output(NewsPageOutput::ToggleSidebar).unwrap()
                        }
                    },
                },
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_propagate_natural_height: true,

                    adw::Clamp {
                        set_maximum_size: 1400,
                        set_tightening_threshold: 1000,

                        #[name = "main_layout"]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 32,
                            set_margin_all: 20,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 24,
                                set_margin_all: 6,

                                gtk::Frame {
                                    set_width_request: 450,
                                    set_height_request: 350,
                                    set_hexpand: false,
                                    set_vexpand: false,
                                    add_css_class: "news-tile-image-frame",

                                    #[name = "thumbnail"]
                                    gtk::Picture {
                                        set_content_fit: gtk::ContentFit::Cover,
                                        add_css_class: "news-tile-image",
                                    }
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 16,
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Start,

                                    gtk::Label {
                                        set_label: model.article.title().as_str(),
                                        set_wrap: true,
                                        set_xalign: 0.0,
                                        set_justify: gtk::Justification::Fill,
                                        add_css_class: "title-1",
                                    },

                                    #[name = "content_label"]
                                    gtk::Label {
                                        set_label: model.article.content().unwrap_or_default().as_str(),
                                        set_wrap: true,
                                        set_xalign: 0.0,
                                        set_justify: gtk::Justification::Fill,
                                        set_visible: model.article.content().is_some(),
                                        add_css_class: "dim-label",
                                        add_css_class: "document",
                                    },

                                    gtk::Button {
                                        set_label: "Open Link",
                                        set_halign: gtk::Align::Start,
                                        add_css_class: "suggested-action",
                                        add_css_class: "pill",
                                    }
                                },
                            },

                        }
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (article, related_articles, cache, show_sidebar_toggle_btn) = init;

        let mut related_section = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 12))
            .detach();

        {
            let mut guard = related_section.guard();
            guard.push_back((
                "Related".to_string(),
                related_articles.clone(),
                cache.clone(),
            ));
        }

        let model = Self {
            show_sidebar_toggle_btn,
            article,
            cache,
            related_section,
        };

        let widgets = view_output!();

        let factory_box = model.related_section.widget();

        factory_box.set_vexpand(true);
        factory_box.set_hexpand(true);
        factory_box.set_valign(gtk::Align::Fill);
        widgets.main_layout.append(factory_box);

        if let Some(url) = &model.article.url_to_image() {
            let loader = crate::utils::image_loader::ImageLoader::new();
            loader.load_picture_image(&widgets.thumbnail, Some(url.clone()), model.cache.clone());
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            NewsPageInput::ShowSidebarToggleBtn(visible) => {
                self.show_sidebar_toggle_btn = visible;
            }
        }
    }
}
