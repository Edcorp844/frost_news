use adw::prelude::*;
use std::sync::Arc;

use relm4::{
    FactorySender,
    prelude::{DynamicIndex, FactoryComponent, FactoryVecDeque},
};

use crate::{
    components::news_tile::NewsTile,
    types::{cache::ImageCache, news_article::NewsArticle},
};

#[derive(Debug)]
pub struct CategorisedNewsSection {
    category: String,
    articles: Vec<Arc<dyn NewsArticle>>,
    tiles: FactoryVecDeque<NewsTile>,
    cache: ImageCache,
    grid: gtk::FlowBox,
}

#[derive(Debug)]
pub enum SectionInput {
    Initialize,
    RequestNewspage((Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>)),
}

#[relm4::factory(pub)]
impl FactoryComponent for CategorisedNewsSection {
    type Init = (String, Vec<Arc<dyn NewsArticle>>, ImageCache);
    type Input = SectionInput;
    type Output = (Arc<dyn NewsArticle>, Vec<Arc<dyn NewsArticle>>);
    type ParentWidget = gtk::Box;
    type CommandOutput = ();

    view! {
        gtk::Box{
             set_orientation: gtk::Orientation::Vertical,

             #[name = "header"]
             gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,

                gtk::Label {
                    set_label: &self.category.to_uppercase(),
                    add_css_class: "section-label",
                },

                #[name = "chevron"]
                gtk::Image {
                    set_icon_name: Some("pan-down-symbolic"),
                    set_pixel_size: 25
                }

             },

             #[name = "revealer"]
             gtk::Revealer {
                set_reveal_child: true,
                set_margin_bottom: 20,
                set_margin_top: 20,

                #[wrap(Some)]
                set_child =  &self.grid.clone(),
             }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let (category, articles, cache) = init;
        sender.input(SectionInput::Initialize);
        let grid = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .column_spacing(12)
            .row_spacing(12)
            .homogeneous(true)
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
            gtk::glib::ControlFlow::Continue
        });

        println!("Initialized for {category}");
        let tiles = FactoryVecDeque::builder()
            .launch(grid.clone())
            .forward(sender.input_sender(), move |data| {
                SectionInput::RequestNewspage(data)
            });

        let model = Self {
            category,
            articles,
            tiles,
            cache,
            grid,
        };

        model
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: FactorySender<Self>,
    ) {
        self.update(msg, sender);
        let header = widgets.header.clone();
        let chevron = widgets.chevron.clone();
        let revealer = widgets.revealer.clone();
        header.set_widget_name(&self.category);

        let header_gesture = gtk::GestureClick::new();

        header_gesture.connect_released(move |_, _, _, _| {
            let is_now_revealing = !revealer.reveals_child();
            revealer.set_reveal_child(is_now_revealing);

            if is_now_revealing {
                chevron.set_icon_name(Some("pan-down-symbolic"));
            } else {
                chevron.set_icon_name(Some("pan-end-symbolic"));
            }
        });
        header.add_controller(header_gesture);

        let mut guard = self.tiles.guard();
        guard.clear();
        for article in self.articles.clone() {
            let mut related = self.articles.clone();
            related.remove(
                self.articles
                    .clone()
                    .iter()
                    .position(|a| a.title() == article.title())
                    .unwrap_or(0),
            );
            guard.push_back((article, related, self.cache.clone()));
        }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            SectionInput::Initialize => {}
            SectionInput::RequestNewspage(data) => {
                let _ = sender.output_sender().send(data);
            }
        }
    }
}
