use adw::prelude::*;
use chrono::{DateTime, Local, Utc};
use relm4::{Component, ComponentParts, ComponentSender, Controller, prelude::*};
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::components::history_bucket::{HistoryBucket, HistoryBucketInput, HistoryBucketOutput};
use crate::services::workers::history_worker::{HistoryWorker, HistoryWorkerInput};
use crate::types::persistent_articel::PersistentArticle;

#[derive(Debug)]
pub struct HistoryPage {
    history_worker: Arc<Controller<HistoryWorker>>,
    show_sidebar_toggle_btn: bool,
    navigation_view: adw::NavigationView,
    sections: FactoryVecDeque<HistoryBucket>,
    history_available: bool,
    select_mode_on: bool,
}

#[derive(Debug)]
pub enum HistoryPageInput {
    UpdateHistory(Vec<Arc<PersistentArticle>>),
    DeleteHistoryEntry(String),
    ShowSidebarToggleBtn(bool),
    ClearHistory,
    ActivateSelectionMode,
    DeactivateSelectMode,
    Dummy,
}

#[derive(Debug)]
pub enum HistoryPagePageOutput {
    ToggleSidebar,
}

#[relm4::component(pub)]
impl Component for HistoryPage {
    type Init = (Arc<Controller<HistoryWorker>>, bool);
    type Input = HistoryPageInput;
    type Output = HistoryPagePageOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &model.navigation_view.clone() {
                 push = &adw::NavigationPage {
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &gtk::Box{
                            set_orientation: gtk::Orientation::Vertical,

                            adw::HeaderBar {
                                set_show_title: true,
                                pack_start = &gtk::Button {
                                    set_icon_name: "sidebar-show-symbolic",
                                    #[watch]
                                    set_visible: model.show_sidebar_toggle_btn,
                                    connect_clicked[sender] => move |_| {
                                        let _ = sender.output(HistoryPagePageOutput::ToggleSidebar);
                                    },
                                },


                            },

                            adw::Clamp{
                                set_margin_bottom: 16,

                                gtk::Box{
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_halign: gtk::Align::End,
                                    set_margin_horizontal: 40,
                                    set_spacing: 10,

                                    gtk::Button {
                                        add_css_class: "destructive-action",
                                        add_css_class: "circular",
                                        set_halign: gtk::Align::End,
                                        #[watch]
                                        set_visible: model.history_available.clone(),



                                        adw::ButtonContent {
                                            set_icon_name: "edit-clear-all-symbolic",
                                            set_label: "Clear",
                                            set_margin_all: 10,
                                            set_halign: gtk::Align::Start,
                                        },

                                        connect_clicked[sender] => move |_| {
                                            sender.input(HistoryPageInput::ClearHistory);
                                        }
                                    },

                                    gtk::Button {
                                        add_css_class: "circular",
                                        set_halign: gtk::Align::End,
                                        #[watch]
                                        set_visible: model.history_available.clone(),



                                        adw::ButtonContent {
                                            set_icon_name: "selection-mode-symbolic",
                                            set_label: "Select",
                                            set_margin_all: 10,
                                            set_halign: gtk::Align::Start,
                                        },

                                        connect_clicked[sender] => move |_| {
                                            sender.input(HistoryPageInput::ActivateSelectionMode);
                                        }
                                    },


                                },

                            },

                        },

                        #[wrap(Some)]
                        set_content = &gtk::ScrolledWindow {
                            set_hscrollbar_policy: gtk::PolicyType::Never,

                            adw::Clamp {
                                set_margin_top: 20,
                                set_margin_bottom: 40,

                                gtk::Box{
                                    set_orientation: gtk::Orientation::Vertical,


                                    gtk::Label{
                                        set_label: "History",
                                        set_xalign: 0.0,
                                        add_css_class: "frost-brand-title",
                                        set_margin_bottom: 20,
                                    },



                                    gtk::Box{
                                        #[watch]
                                        set_visible: !model.history_available.clone(),

                                        gtk::Label{
                                            set_label: "No History Availbe."
                                        }
                                    },

                                    #[local_ref]
                                    sections_widget -> gtk::Box {
                                        #[watch]
                                        set_visible: model.history_available.clone(),
                                    }
                                }

                            }
                        },
                    },
                 },
            }
        },
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (history_worker, show_sidebar_toggle_btn) = init;

        history_worker.emit(HistoryWorkerInput::Subscribe(sender.clone()));
        history_worker.emit(HistoryWorkerInput::Fetch);

        let navigation_view = adw::NavigationView::builder().build();

        let sections = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 12))
            .forward(sender.input_sender(), move |message| match message {
                HistoryBucketOutput::DeleteEntry(id) => HistoryPageInput::DeleteHistoryEntry(id),
                HistoryBucketOutput::Dummy => HistoryPageInput::Dummy,
            });

        let model = HistoryPage {
            history_worker,
            show_sidebar_toggle_btn,
            navigation_view,
            sections,
            history_available: false,
            select_mode_on: false,
        };

        let sections_widget = model.sections.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            HistoryPageInput::UpdateHistory(articles) => {
                self.history_available = true;
                let mut guard = self.sections.guard();
                guard.clear();

                if articles.is_empty() {
                    self.history_available = false;
                    return;
                }

                let mut groups: BTreeMap<i64, (String, Vec<Arc<PersistentArticle>>)> =
                    BTreeMap::new();

                let now = Local::now();
                let today = now.date_naive();

                for article in articles {
                    let dt = DateTime::<Utc>::from_timestamp(article.visit_time, 0)
                        .map(|u| u.with_timezone(&Local))
                        .unwrap_or(now);

                    let article_date = dt.date_naive();
                    let days_diff = (today - article_date).num_days();

                    let (label, sort_key) = match days_diff {
                        0 => ("Today".to_string(), 0),
                        1 => ("Yesterday".to_string(), 1),
                        2..=6 => (dt.format("%A").to_string(), days_diff),
                        _ => (dt.format("%A, %B %d").to_string(), dt.timestamp() * -1),
                    };

                    groups
                        .entry(sort_key)
                        .or_insert((label, Vec::new()))
                        .1
                        .push(article);
                }

                for (_key, (label, items)) in groups {
                    guard.push_back((label, items));
                }
            }
            HistoryPageInput::ShowSidebarToggleBtn(visible) => {
                self.show_sidebar_toggle_btn = visible;
            }
            HistoryPageInput::DeleteHistoryEntry(id) => {
                self.history_worker.emit(HistoryWorkerInput::Delete(id));
            }
            HistoryPageInput::ClearHistory => {
                self.history_worker.emit(HistoryWorkerInput::DeleterAll);
                self.history_available = true;
            }
            HistoryPageInput::ActivateSelectionMode => {
                self.select_mode_on = true;
                self.sections.broadcast(HistoryBucketInput::ActivateSelectionMode);
            }
            HistoryPageInput::DeactivateSelectMode=>{
                self.select_mode_on = false;
            }
            HistoryPageInput::Dummy => {}
        }

        self.update_view(widgets, sender);
    }
}
