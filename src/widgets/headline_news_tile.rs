use crate::{
    types::cache::ImageCache, ui_build_herlper_functions::image_functions::image_loader,
    widgets::blurred_image::BlurredBottomImage,
};
use gtk::prelude::*;

pub struct HeadlineNewsTile {}

impl HeadlineNewsTile {
    pub fn widget(
        title: &str,
        image_url: Option<String>,
        published_at: String,
        desc: Option<String>,
        cache: ImageCache,
        on_click: impl Fn() + 'static,
    ) -> gtk::Widget {
        let shared_click = std::sync::Arc::new(on_click);
        let click_clone1 = shared_click.clone();

        let tile = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .width_request(400)
            .height_request(400)
            .css_classes(vec!["headline-tile"])
            .build();

        let tile_gesture = gtk::GestureClick::new();

        tile_gesture.connect_released(move |_, _, _, _| {
            click_clone1();
        });

        tile.add_controller(tile_gesture.clone());

        let overlay = gtk::Overlay::new();

        let image_container = gtk::Frame::builder()
            .overflow(gtk::Overflow::Hidden)
            .css_classes(vec!["headline-image-frame"])
            .build();

        let image = BlurredBottomImage::new();
        image.set_content_fit(gtk::ContentFit::Cover);

        image_container.set_child(Some(image.widget()));
        overlay.set_child(Some(&image_container));

        let info_overlay = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .valign(gtk::Align::End)
            .css_classes(vec!["image-scrim"])
            .build();

        let title_label = gtk::Label::builder()
            .label(title)
            .wrap(true)
            .lines(3)
            .xalign(0.0)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .vexpand(true)
            .css_classes(vec!["headline-title"])
            .build();

        info_overlay.append(&title_label);

        let display_date = chrono::DateTime::parse_from_rfc3339(&published_at)
            .map(|dt| dt.format("%b %d").to_string())
            .unwrap_or_else(|_| published_at.clone());

        if let Some(desc) = desc {
            let meta_label = gtk::Label::builder()
                .label(desc)
                .wrap(true)
                .xalign(0.0)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .css_classes(vec!["news-tile-description"])
                .build();

            info_overlay.append(&meta_label);
        }

        let tile_action_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Center)
            .build();

        let date = gtk::Label::builder()
            .label(format!("| {}",display_date.as_str()))
            .wrap(true)
            .xalign(0.0)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .css_classes(vec!["news-tile-description"])
            .build();

        tile_action_container.append(&date);

        let spacer = gtk::Separator::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(vec!["spacer"])
            .build();

        tile_action_container.append(&spacer);

        let read_more_action_button = gtk::Button::builder()
            .label("Read More")
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(vec!["suggested-action", "read-more-button"])
            .build();

        let btn_gesture = gtk::GestureClick::new();
        let click_clone2 = shared_click.clone();
        btn_gesture.connect_released(move |_, _, _, _| {
            click_clone2();
        });

        read_more_action_button.add_controller(btn_gesture.clone());

       // tile_action_container.append(&read_more_action_button);
        info_overlay.append(&tile_action_container);
        overlay.add_overlay(&info_overlay);
        tile.append(&overlay);

        // Load image for BlurredBottomImage
        let image_loader = image_loader::ImageLoader::new();
        image_loader.load_blurred_image(&image, image_url, cache);

        tile.upcast()
    }
}
