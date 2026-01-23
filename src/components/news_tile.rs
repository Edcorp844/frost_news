use crate::types::cache::ImageCache;
use crate::types::news_article::NewsArticle;
use gtk::{pango, prelude::*};
use relm4::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct NewsTile {
    article: Arc<dyn NewsArticle>,
    cache: ImageCache,
    related_articles: Vec<Arc<dyn NewsArticle>>,
}

#[derive(Debug)]
pub enum NewsTileInput {
    Clicked,
}

#[relm4::factory(pub)]
impl FactoryComponent for NewsTile {
    type Init = (Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>, ImageCache);
    type Input = NewsTileInput;
    type Output = (Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>);
    type ParentWidget = gtk::FlowBox;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 6,
            add_css_class: "news-grid-tile",

            gtk::Frame {
                add_css_class: "news-tile-image-frame",
                set_height_request: 200,
                set_margin_bottom: 12,

                #[name = "thumbnail"]
                gtk::Picture {
                    set_content_fit: gtk::ContentFit::Cover,
                    add_css_class: "news-tile-image",
                }
            },

            gtk::Label {
                set_label: &self.article.title(),
                set_wrap: true,
                set_lines: 4,
                set_ellipsize: pango::EllipsizeMode::End,
                //
                set_xalign: 0.0,
                set_margin_bottom: 6,
                set_margin_horizontal: 2,
                add_css_class: "news-tile-title",
            },

            gtk::Label {
                set_label: self.article.description().as_deref().unwrap_or(""),
                set_wrap: true,
                set_lines: 3,
                set_ellipsize: pango::EllipsizeMode::End,
               // set_justify: gtk::Justification::Fill,
                set_xalign: 0.0,
                set_margin_horizontal: 2,
                set_margin_bottom: 6,
                add_css_class: "dim-label",
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                add_css_class: "spacer",
            },

            gtk::Label {
                #[watch]
                set_label: chrono::DateTime::parse_from_rfc3339(&self.article.published_at()).map(|dt| dt.format("%b %d").to_string()).unwrap_or_else(|_| self.article.published_at().clone()).as_str(),
                set_xalign: 0.0,
                set_margin_horizontal: 2,
                add_css_class: "news-tile-date",
            },

            add_controller = gtk::GestureClick {
                connect_released[sender] => move |_, _, _, _| {
                    sender.input(NewsTileInput::Clicked);
                }
            },


        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let (article, related_articles, cache) = init;

        Self {
            article,
            cache,
            related_articles,
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: FactorySender<Self>,
    ) {
        self.update(msg, sender);
        if let Some(url) = &self.article.url_to_image() {
            let loader = crate::utils::image_loader::ImageLoader::new();
            loader.load_picture_image(&widgets.thumbnail, Some(url.clone()), self.cache.clone());
        }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            NewsTileInput::Clicked => {
                let _ = sender
                    .output_sender()
                    .send((self.article.clone(), self.related_articles.clone()));
            }
        }
    }
}
