use std::sync::Arc;

use relm4::{ComponentSender, Worker};

use crate::{
    components::sidebar::{SideBar, SidebarInput},
    pages::history_page::{HistoryPage, HistoryPageInput},
    services::history_service::history::HistoryService,
    types::persistent_articel::PersistentArticle,
};

#[derive(Debug)]
pub struct HistoryWorker {
    service: HistoryService,
    // List of senders to notify (the Sidebar, etc.)
    subscribers: Vec<relm4::ComponentSender<HistoryPage>>,
}

#[derive(Debug)]
pub enum HistoryWorkerInput {
    Fetch,
    DeleterAll,
    Save(PersistentArticle),
    Delete(String),
    Subscribe(relm4::ComponentSender<HistoryPage>),
}

impl Worker for HistoryWorker {
    type Init = HistoryService;
    type Input = HistoryWorkerInput;
    type Output = Vec<Arc<PersistentArticle>>;

    fn init(service: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self {
            service,
            subscribers: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            HistoryWorkerInput::Fetch => {
                if let Ok(recent) = self.service.get_all_history() {
                    for sub in &self.subscribers {
                        sub.input(HistoryPageInput::UpdateHistory(recent.clone()));
                    }
                }
            }
            HistoryWorkerInput::DeleterAll => {
                if let Ok(_) = self.service.clear_all_history() {
                    if let Ok(updated_history) = self.service.get_all_history() {
                        for subscriber in &self.subscribers {
                            let _ = subscriber
                                .input(HistoryPageInput::UpdateHistory(updated_history.clone()));
                        }
                    }
                }
            }
            HistoryWorkerInput::Save(article) => {
                let _ = self.service.save_to_history(article);

                if let Ok(recent) = self.service.get_all_history() {
                    for sub in &self.subscribers {
                        sub.input(HistoryPageInput::UpdateHistory(recent.clone()));
                    }
                }
            }
            HistoryWorkerInput::Delete(id) => {
                if let Ok(_) = self.service.delete_entry(id) {
                    if let Ok(updated_history) = self.service.get_all_history() {
                        for subscriber in &self.subscribers {
                            let _ = subscriber
                                .input(HistoryPageInput::UpdateHistory(updated_history.clone()));
                        }
                    }
                }
            }

            HistoryWorkerInput::Subscribe(sender) => {
                self.subscribers.push(sender);
            }
        }
    }
}
