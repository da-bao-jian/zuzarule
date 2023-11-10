#![allow(dead_code)]
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;
use teloxide::types::{ChatId, Message, MessageId};

lazy_static! {
    pub(crate) static ref GLOBAL_MAIN_MENU_STORAGE: MainMenuStorage = MainMenuStorage::new();
}

lazy_static! {
    pub(crate) static ref GLOBAL_CREATE_PROPOSAL_STORAGE: ProposalMenuStorage =
        TgMessageStorage::new();
}

lazy_static! {
    #[derive(Debug)]
    pub(crate) static ref GLOBAL_PROPOSAL_STORAGE: ProposalStorage = TgProposalStorage::new();
}

pub(crate) trait TgMessageStorage {
    fn new() -> Self;
    fn insert(&self, user_name: String, message: TgMessage);
    fn get(&self, user_name: String) -> Option<TgMessage>;
    fn remove(&self, user_name: String) -> Option<TgMessage>;
    fn delete_all(&self);
}

#[derive(Debug, Clone)]
pub(crate) struct Proposal {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) starting_date: String,
    pub(crate) expiration_date: String,
    pub(crate) vote: u64,
}

pub(crate) trait TgProposalStorage {
    fn new() -> Self;
    fn insert(&self, user_name: String, message: Proposal);
    fn get(&self, user_name: String) -> Option<Vec<Proposal>>;
    fn remove(&self, user_name: String) -> Option<Proposal>;
    fn delete_all(&self);
}

#[derive(Debug, Default)]
pub(crate) struct ProposalStorage {
    storage: Arc<RwLock<HashMap<String, Vec<Proposal>>>>,
}

#[derive(Debug, Clone)]
pub(crate) struct TgMessage {
    pub(crate) chat_id: ChatId,
    pub(crate) message_id: MessageId,
    pub(crate) message: Arc<Message>,
}

impl TgProposalStorage for ProposalStorage {
    fn new() -> Self {
        ProposalStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, proposal: Proposal) {
        let mut storage = self.storage.write();
        storage
            .entry(user_name)
            .or_insert_with(Vec::new)
            .push(proposal);
    }

    fn get(&self, user_name: String) -> Option<Vec<Proposal>> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, _user_name: String) -> Option<Proposal> {
        todo!()
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}

#[derive(Debug, Default)]
pub(crate) struct ProposalMenuStorage {
    storage: Arc<RwLock<HashMap<String, TgMessage>>>,
}

impl TgMessageStorage for ProposalMenuStorage {
    fn new() -> Self {
        ProposalMenuStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, message: TgMessage) {
        let mut storage = self.storage.write();
        storage.insert(user_name, message);
    }

    fn get(&self, user_name: String) -> Option<TgMessage> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, user_name: String) -> Option<TgMessage> {
        let mut storage = self.storage.write();
        storage.remove(&user_name)
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}

#[derive(Debug, Default)]
pub(crate) struct MainMenuStorage {
    storage: Arc<RwLock<HashMap<String, TgMessage>>>,
}

impl TgMessageStorage for MainMenuStorage {
    fn new() -> Self {
        MainMenuStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, message: TgMessage) {
        let mut storage = self.storage.write();
        storage.insert(user_name, message);
    }

    fn get(&self, user_name: String) -> Option<TgMessage> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, user_name: String) -> Option<TgMessage> {
        let mut storage = self.storage.write();
        storage.remove(&user_name)
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}
