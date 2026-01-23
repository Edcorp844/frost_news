use adw::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender, prelude::*};

use crate::{data::sections::SECTIONS, types::news_category::NewsSection};

#[derive(Debug)]
pub enum NavigationPage {
    Saved,
    History,
    Settings,
    CustomEndpoint(),
    Category(String),
}

impl NavigationPage {
    pub fn to_key(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct SideBar {}

#[derive(Debug)]
pub enum SidebarMessage {
    ToggleSidebar,
    SelectSection(NewsSection),
    SelectPage(NavigationPage),
}

#[derive(Debug)]
pub enum SidebarInput {}

#[relm4::component(pub)]
impl Component for SideBar {
    type Init = ();
    type Input = SidebarInput;
    type Output = SidebarMessage;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: "FrostNews",
            set_width_request: 300,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,

                    adw::HeaderBar {
                        set_show_title: false,
                        pack_end = &gtk::Button {
                            set_icon_name: "sidebar-show-symbolic",
                            set_tooltip: "Hide Sidebar",
                            add_css_class: "flat",
                            connect_clicked[sender] => move |_| {
                                let _ = sender.output(SidebarMessage::ToggleSidebar);
                            }
                        }
                    },

                    gtk::SearchEntry {
                        set_placeholder_text: Some("Search..."),
                        set_margin_horizontal: 16,
                    },

                    #[name = "topheadlineslist"]
                    gtk::ListBox {
                        set_selection_mode: gtk::SelectionMode::Single,
                        set_margin_horizontal: 12,
                        set_margin_bottom: 10,
                        add_css_class: "navigation-sidebar",

                        gtk::ListBoxRow {
                            set_margin_start: 0,
                            set_margin_end: 0,
                            gtk::Box {
                                set_spacing: 16,
                                add_css_class: "Category",

                                gtk::Image {
                                    set_icon_name: Some("emoji-objects-symbolic"),
                                    set_pixel_size: 18,
                                    set_margin_start: 8,
                                    add_css_class: "sidebar_icon",
                                },
                                gtk::Label {
                                    set_label: "Top Headlines",
                                    add_css_class: "sidebar-label",
                                }
                            }
                        }
                    }
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,

                        // --- Library Section ---
                        #[name = "library_header"]
                        gtk::Box {
                            add_css_class: "sidebar-header-box",
                            set_margin_horizontal: 20,
                            gtk::Label {
                                set_label: "Library",
                                add_css_class: "sidebar-section-title",
                                add_css_class: "dimmed",
                            },
                            gtk::Separator { set_hexpand: true, add_css_class: "spacer" },
                            #[name = "library_chevron"]
                            gtk::Image { set_icon_name: Some("pan-down-symbolic"), add_css_class: "dimmed" }
                        },
                        #[name = "library_revealer"]
                        gtk::Revealer {
                            set_reveal_child: true,
                            #[name = "library"]
                            gtk::ListBox {
                                // Start with None to prevent auto-selection during population
                                set_selection_mode: gtk::SelectionMode::None,
                                set_margin_horizontal: 12,
                                add_css_class: "navigation-sidebar"
                            }
                        },

                         // --- Custom End Point---
                        gtk::Box {
                            set_visible: false,

                            #[name = "custom_endpoints_header"]
                            gtk::Box {
                                add_css_class: "sidebar-header-box",
                                set_margin_horizontal: 20,
                                gtk::Label {
                                    set_label: "Custom Endpoints",
                                    add_css_class: "sidebar-section-title",
                                    add_css_class: "dimmed",
                                },
                                gtk::Separator { set_hexpand: true, add_css_class: "spacer" },
                                #[name = "custom_endpoints_chevron"]
                                gtk::Image { set_icon_name: Some("pan-down-symbolic"), add_css_class: "dimmed" }
                            },
                            #[name = "custom_endpoints_revealer"]
                            gtk::Revealer {
                                set_reveal_child: true,
                                #[name = "custom_endpoints"]
                                gtk::ListBox {
                                    // Start with None to prevent auto-selection during population
                                    set_selection_mode: gtk::SelectionMode::None,
                                    set_margin_horizontal: 12,
                                    add_css_class: "navigation-sidebar"
                                }
                            },
                        },

                        // --- Categories ---
                        gtk::Box{
                            set_visible: false,

                            #[name = "categories_header"]
                            gtk::Box {
                                add_css_class: "sidebar-header-box",
                                set_margin_horizontal: 20,
                                gtk::Label {
                                    set_label: "Categories",
                                    add_css_class: "sidebar-section-title",
                                    add_css_class: "dimmed",
                                },
                                gtk::Separator { set_hexpand: true, add_css_class: "spacer" },
                                #[name = "categories_chevron"]
                                gtk::Image { set_icon_name: Some("pan-down-symbolic"), add_css_class: "dimmed" }
                            },
                            #[name = "categories_revealer"]
                            gtk::Revealer {
                                set_reveal_child: true,
                                #[name = "categories"]
                                gtk::ListBox {
                                    // Start with None to prevent auto-selection during population
                                    set_selection_mode: gtk::SelectionMode::None,
                                    set_margin_horizontal: 12,
                                    add_css_class: "navigation-sidebar"
                                }
                            },

                        },


                        // --- sections Section ---
                        #[name = "section_header"]
                        gtk::Box {
                            add_css_class: "sidebar-header-box",
                            set_margin_horizontal: 20,
                            gtk::Label {
                                set_label: "Sections",
                                add_css_class: "sidebar-section-title",
                                add_css_class: "dimmed",
                            },
                            gtk::Separator { set_hexpand: true, add_css_class: "spacer" },
                            #[name = "section_chevron"]
                            gtk::Image { set_icon_name: Some("pan-down-symbolic"), add_css_class: "dimmed" }
                        },
                        #[name = "sections_revealer"]
                        gtk::Revealer {
                            set_reveal_child: true,
                            #[name = "sections"]
                            gtk::ListBox {
                                set_selection_mode: gtk::SelectionMode::None,
                                set_margin_horizontal: 12,
                                add_css_class: "navigation-sidebar"
                            }
                        },


                    }
                },

                add_bottom_bar = &gtk::Box {
                    set_margin_all: 12,
                    set_orientation: gtk::Orientation::Horizontal,
                    
                    gtk::Button {
                        add_css_class: "flat",
                        set_hexpand: true, 
                        
                        adw::ButtonContent {
                            set_icon_name: "emblem-system-symbolic",
                            set_label: "Settings",
                            set_halign: gtk::Align::Start, 
                        },
                        
                        connect_clicked[sender] => move |_| {
                            let _ = sender.output(SidebarMessage::SelectPage(NavigationPage::Settings));
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SideBar {};

        let widgets = view_output!();

        Self::populate_sections(&widgets, &sender);
        Self::render_library_list(&widgets, &sender);

        widgets.library.set_can_focus(false);
        widgets.sections.set_can_focus(false);

        widgets
            .library
            .set_selection_mode(gtk::SelectionMode::Single);
        widgets
            .sections
            .set_selection_mode(gtk::SelectionMode::Single);

        widgets.library.unselect_all();
        widgets.sections.unselect_all();

        let w_cat = widgets.sections.clone();
        let w_lib = widgets.library.clone();
        widgets.topheadlineslist.connect_row_activated(move |_, _| {
            w_cat.unselect_all();
            w_lib.unselect_all();
        });

        if let Some(row) = widgets
            .topheadlineslist
            .first_child()
            .and_then(|w| w.dynamic_cast::<gtk::ListBoxRow>().ok())
        {
            widgets.topheadlineslist.select_row(Some(&row));
        }

        widgets
            .topheadlineslist
            .clone()
            .connect_row_activated(move |_, _| {
                let _ = sender
                    .output_sender()
                    .send(SidebarMessage::SelectSection(NewsSection::General));
            });

        Self::setup_collapsible_section(
            &widgets.section_header,
            &widgets.sections_revealer,
            &widgets.section_chevron,
        );
        Self::setup_collapsible_section(
            &widgets.library_header,
            &widgets.library_revealer,
            &widgets.library_chevron,
        );

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        _widgets: &mut Self::Widgets,
        _msg: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
    }
}

impl SideBar {
    fn populate_sections(widgets: &SideBarWidgets, sender: &ComponentSender<Self>) {
        let sections_list = &widgets.sections;

        for cat in SECTIONS {
            let row_box = gtk::Box::builder()
                .spacing(16)
                .css_classes(vec!["Category"])
                .build();

            let icon = gtk::Image::from_icon_name(cat.icon);
            icon.set_pixel_size(18);
            icon.set_margin_start(8);
            icon.add_css_class("sidebar_icon");

            let label = gtk::Label::builder()
                .label(cat.name)
                .css_classes(vec!["sidebar-label"])
                .build();

            row_box.append(&icon);
            row_box.append(&label);

            let row = gtk::ListBoxRow::builder()
                .child(&row_box)
                .name(cat.id)
                .margin_end(0)
                .margin_start(0)
                .build();

            sections_list.append(&row);
        }

        let s = sender.clone();
        let w_headlines = widgets.topheadlineslist.clone();
        let w_library = widgets.library.clone();

        sections_list.connect_row_activated(move |_, row| {
            w_headlines.unselect_all();
            w_library.unselect_all();

            let id = row.widget_name();
            if let Some(matched) = SECTIONS.iter().find(|c| c.id == id) {
                let _ = s.output(SidebarMessage::SelectSection(matched.enum_name.clone()));
            }
        });
    }

    fn render_library_list(widgets: &SideBarWidgets, sender: &ComponentSender<Self>) {
        let listbox = &widgets.library;
        let items = [
            ("user-bookmarks-symbolic", "Saved"),
            ("document-open-recent-symbolic", "History"),
        ];

        for (icon_name, label_text) in items {
            let row_box = gtk::Box::builder()
                .spacing(16)
                .css_classes(vec!["Category"])
                .build();

            let icon = gtk::Image::from_icon_name(icon_name);
            icon.set_pixel_size(18);
            icon.set_margin_start(8);
            icon.add_css_class("sidebar_icon");

            let label = gtk::Label::builder()
                .label(label_text)
                .css_classes(vec!["sidebar-label"])
                .build();

            row_box.append(&icon);
            row_box.append(&label);

            let row = gtk::ListBoxRow::builder()
                .name(label_text)
                .child(&row_box)
                .margin_end(0)
                .margin_start(0)
                .build();

            listbox.append(&row);
        }

        let w_headlines = widgets.topheadlineslist.clone();
        let w_sections = widgets.sections.clone();

        let sender_clone = sender.clone();
        listbox.connect_row_activated(move |_, row| {
            w_headlines.unselect_all();
            w_sections.unselect_all();

            let page = if row.widget_name().as_str() == "Saved" {
                NavigationPage::Saved
            } else {
                NavigationPage::History
            };
            let _ = sender_clone
                .output_sender()
                .send(SidebarMessage::SelectPage(page));
        });
    }

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
