use gtk::{Image, Label, pango, prelude::*};
use relm4::{prelude::*, view};
use std::sync::Arc;

use crate::types::persistent_articel::PersistentArticle;

#[derive(Debug)]
pub struct HistoryRow {
    article: Arc<PersistentArticle>,
    selection_mode_on: bool,
    selected: bool,
}

#[derive(Debug, Clone)]
pub enum HistoryRowInput {
    ShowMenu,
    SeletctMode(bool),
    Select,
    Deselect,
    ActivateSelectAll,
    DeactivateSelectAll,
    ActivateSelectionMode,
}

#[derive(Debug)]
pub enum HistoryRowOutput {
    Delete(String),
    Selected(String),
    Deselected(String),
    DeactivateSelectAll,
}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryRow {
    type Init = Arc<PersistentArticle>;
    type Input = HistoryRowInput;
    type Output = HistoryRowOutput;
    type ParentWidget = gtk::ListBox;
    type CommandOutput = ();

    view! {
        #[name = "row"]
        gtk::ListBoxRow {
            set_activatable: true,

            gtk::Box{
                set_spacing: 16,
                set_margin_all: 10,

                 gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_visible: self.selection_mode_on,

                    gtk::CheckButton{
                        add_css_class: "selection-mode",
                        connect_toggled[sender] => move |btn|{
                            if btn.is_active() {
                                sender.input(HistoryRowInput::Select);
                            } else {
                                 sender.input(HistoryRowInput::Deselect);
                            }

                        }
                    }

                 },

                gtk::Frame{
                    set_height_request: 100,
                    set_width_request: 100,


                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Start,

                    gtk::Label{
                        set_label: &self.article.title.as_str(),
                        set_wrap: true,
                        set_lines: 2,
                        set_xalign: 0.0,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        add_css_class: "document"
                    },

                    gtk::Label{
                        set_label: &self.article.description.as_deref().unwrap_or(""),
                        set_wrap: true,
                        set_lines: 2,
                        set_xalign: 0.0,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        add_css_class: "document",
                        add_css_class: "dim-label",
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    #[name="actions_button"]
                    gtk::Button{
                        set_icon_name: "view-more-horizontal-symbolic",
                        add_css_class: "circular",
                        //add_css_class: "",
                        #[watch]
                        set_sensitive: !self.selection_mode_on,
                        connect_clicked[sender] => move |_| {
                        sender.input(HistoryRowInput::ShowMenu);
                        }
                    }
                }

            }
        }
    }

    fn init_model(article: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        Self {
            article,
            selection_mode_on: false,
            selected: false,
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: FactorySender<Self>,
    ) {
        match message {
            HistoryRowInput::ShowMenu => {
                let button = widgets.actions_button.clone();
                Self::show_menu(self.article.id.clone(), &button, sender.clone());
            }
            HistoryRowInput::SeletctMode(on) => {
                self.selection_mode_on = on;
            }
            HistoryRowInput::Select => {
                self.selected = true;
                let _ = sender.output(HistoryRowOutput::Selected(self.article.id.clone()));
                println!("Selected: {}", self.selected);
            }
            HistoryRowInput::Deselect => {
                self.selected = false;
                let _ = sender.output(HistoryRowOutput::Deselected(self.article.id.clone()));
                println!("Selected: {}", self.selected);
            }
            HistoryRowInput::ActivateSelectAll => {
                if !self.selection_mode_on {
                    return;
                }

                self.selected = true;
            }
            HistoryRowInput::DeactivateSelectAll => {
                if !self.selection_mode_on {
                    return;
                }
                self.selected = false;
            }
            HistoryRowInput::ActivateSelectionMode => {
                self.selection_mode_on = true;
            }
        }

        self.update(message, sender);
    }
}

impl HistoryRow {
    fn show_menu(entry_id: String, button: &gtk::Button, sender: FactorySender<Self>) {
        let menu_list = gtk::ListBox::builder().build();

        let options = vec![
            (Some("edit-delete-symbolic"), "Delete"),
            (Some("edit-copy-symbolic"), "Copy"),
            (None, "Copy Title"),
            (None, "Delete"),
        ];

        for (icon, label) in options {
            let row = &Self::menu_row(icon, label);
            row.set_activatable(true);
            let sender_clone = sender.clone();
            let id = entry_id.clone();
            row.connect_activate(move |row| {
                println!("Deleting {}", row.widget_name());
                if row.widget_name() == "Delete" {
                    println!("Deleting {}", id);
                    let _ = sender_clone
                        .clone()
                        .output(HistoryRowOutput::Delete(id.clone()));
                }
            });
            menu_list.append(row);
        }
        let menu = gtk::Popover::builder()
            .child(&menu_list)
            .css_classes(vec!["menu"])
            .build();

        menu.set_parent(button);

        menu.popup();
    }

    fn menu_row(icon: Option<&str>, label: &str) -> gtk::ListBoxRow {
        let row = gtk::Box::builder()
            .spacing(16)
            .css_classes(vec!["Category"])
            .build();

        if let Some(icon) = icon {
            let icon = gtk::Image::from_icon_name(icon);
            icon.set_pixel_size(18);
            icon.set_margin_start(8);
            icon.add_css_class("sidebar_icon");

            row.append(&icon);
        }

        let lab = gtk::Label::builder()
            .label(label)
            .css_classes(vec!["sidebar-label"])
            .build();

        row.append(&lab);

        gtk::ListBoxRow::builder()
            .child(&row)
            .name(label)
            .margin_end(0)
            .margin_start(0)
            .build()
    }
}
