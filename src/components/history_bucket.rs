use gtk::prelude::*;
use relm4::prelude::*;
use std::sync::Arc;

use crate::{
    components::history_row::{HistoryRow, HistoryRowInput, HistoryRowOutput},
    types::persistent_articel::PersistentArticle,
};

#[derive(Debug)]
pub struct HistoryBucket {
    title: String,
    rows: FactoryVecDeque<HistoryRow>,
    articles: Vec<Arc<PersistentArticle>>,
    listbox: gtk::ListBox,
    select_mode_on: bool,
}

#[derive(Debug)]
pub enum HistoryBucketOutput {
    DeleteEntry(String),
    Dummy,
}

#[derive(Debug, Clone)]
pub enum HistoryBucketInput {
    ActivateSelectionMode,
    DeleteEntry(String),
    Dummy,
}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryBucket {
    type Init = (String, Vec<Arc<PersistentArticle>>);
    type Input = HistoryBucketInput;
    type Output = HistoryBucketOutput;
    type ParentWidget = gtk::Box;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[name = "header"]
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                set_hexpand: true,

                // Add the click gesture here
                add_controller = gtk::GestureClick {
                    connect_released[revealer, chevron] => move |_, _, _, _| {
                        let is_revealing = !revealer.reveals_child();
                        revealer.set_reveal_child(is_revealing);
                        chevron.set_icon_name(Some(if is_revealing {
                            "pan-down-symbolic"
                        } else {
                            "pan-end-symbolic"
                        }));
                    }
                },

                gtk::Label {
                    set_label: &self.title.to_uppercase(),
                    add_css_class: "section-label",
                    add_css_class: "dimmed",

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
                set_margin_vertical: 20,

                // Assign the listbox that the factory is actually managing
                #[track = "true"]
                set_child: Some(&self.listbox),
            }
        },
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let (title, articles) = init;

        let listbox = gtk::ListBox::new();
        listbox.add_css_class("boxed-list");
        listbox.set_selection_mode(gtk::SelectionMode::None);

        let mut rows = FactoryVecDeque::builder().launch(listbox.clone()).forward(
            sender.input_sender(),
            move |message| match message {
                HistoryRowOutput::Delete(id) => HistoryBucketInput::DeleteEntry(id),
                _ => HistoryBucketInput::Dummy,
            },
        );

        // 2. Populate the rows immediately inside init_model
        {
            let mut guard = rows.guard();
            for article in &articles {
                guard.push_back(article.clone());
            }
        }

        Self {
            title,
            rows,
            articles,
            listbox,
            select_mode_on: false,
        }
    }

    // Standard update
    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            HistoryBucketInput::DeleteEntry(id) => {
                //self.rows.broadcast(HistoryRowInput::DeactivateSelectAll);
            }
            HistoryBucketInput::Dummy => {
                sender.output(HistoryBucketOutput::Dummy);
            }
            HistoryBucketInput::ActivateSelectionMode => {
                self.rows.broadcast(HistoryRowInput::ActivateSelectionMode);
            }
        }
    }
}

impl HistoryBucket {
    pub fn setup_collapsible_section(
        header: &gtk::Box,
        revealer: &gtk::Revealer,
        chevron: &gtk::Image,
    ) {
        let r = revealer.clone();
        let c = chevron.clone();
        let gesture = gtk::GestureClick::new();

        gesture.connect_released(move |_, _, _, _| {
            let is_revealing = !r.reveals_child();
            r.set_reveal_child(is_revealing);
            c.set_icon_name(Some(if is_revealing {
                "pan-down-symbolic"
            } else {
                "pan-end-symbolic"
            }));
        });
        header.add_controller(gesture);
    }
}
