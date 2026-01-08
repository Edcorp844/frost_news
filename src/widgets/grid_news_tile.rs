use crate::{types::cache::ImageCache, ui_build_herlper_functions::image_functions::image_loader};
use gtk::prelude::*;

pub struct NewsRowTile {}

impl NewsRowTile {
    pub fn widget(
        title: &str,
        desc: Option<String>,
        image_url: Option<String>,
        published_at: String,
        cache: ImageCache,
        on_click: impl Fn() + 'static,
    ) -> gtk::Widget {
        // 1. The Main Container (The Card)
        let tile = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(6)
            .margin_start(6)
            .margin_end(6)
            .css_classes(vec!["news-grid-tile"])
            .build();

        let gesture = gtk::GestureClick::new();
        
        gesture.connect_released(move |_, _, _, _| {
            on_click();
        });

        tile.add_controller(gesture);

        // 2. The Image Section
        let image_frame = gtk::Frame::builder()
            .width_request(150)
            .css_classes(vec!["news-tile-image-frame"])
            .height_request(200)
            .margin_bottom(12)
            .build();

        let image = gtk::Picture::builder()
            .content_fit(gtk::ContentFit::Cover)
            .css_classes(vec!["news-tile-image"])
            .width_request(150)
            .height_request(200)
            .build();

        image_frame.set_child(Some(&image));
        tile.append(&image_frame);

        let title_label = gtk::Label::builder()
            .label(title)
            .wrap(true)
            .lines(4)
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .margin_bottom(6)
            .margin_start(2)
            .margin_end(2)
            .css_classes(vec!["title-4"]) // Uses standard Adwaita heading size
            .build();

        tile.append(&title_label);

        let desc_label = gtk::Label::builder()
            .label(desc.as_deref().unwrap_or(""))
            .wrap(true)
            .lines(3)
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .margin_start(2)
            .margin_end(2)
            .margin_bottom(6)
            .css_classes(vec!["body", "dim-label"]) // 'dim-label' makes it slightly gray
            .build();

        tile.append(&desc_label);

        let v_spacer = gtk::Separator::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .css_classes(vec!["spacer"])
            .build();

        tile.append(&v_spacer);

        let display_date = chrono::DateTime::parse_from_rfc3339(&published_at)
            .map(|dt| dt.format("%b %d").to_string())
            .unwrap_or_else(|_| published_at.clone());

        let date = gtk::Label::builder()
            .label(display_date.as_str())
            .wrap(true)
            .xalign(0.0)
            .margin_start(2)
            .margin_end(2)
            .margin_bottom(12)
            .css_classes(vec!["news-tile-description"])
            .build();

        tile.append(&date);

        // Image Loading logic
        let image_loader = image_loader::ImageLoader::new();
        image_loader.load_picture_image(&image, image_url, cache);

        tile.upcast()
    }
}
