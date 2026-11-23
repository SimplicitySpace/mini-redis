// data_store.rs
use std::{collections::HashMap, io::ErrorKind};

use crate::parser::Action;

#[derive(Clone, Debug)]
pub struct SomeData {
    pub value: String,
}

pub struct Datastore {
    pub store: HashMap<String, SomeData>,
}

#[derive(Debug)]
pub enum ActionResponse {
    Successful,
    Errored(ErrorKind),
    NoteFound(SomeData),
}

impl Datastore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn perform_action(&mut self, action: &Action) -> ActionResponse {
        match action {
            Action::Set { key, value } => {
                self.push_map(
                    key.to_string(),
                    SomeData {
                        value: value.clone(),
                    },
                );
                ActionResponse::Successful
            }
            Action::Get { key } => match self.get_map(key) {
                Ok(data) => ActionResponse::NoteFound(data),
                Err(e) => ActionResponse::Errored(e),
            },
            Action::Delete { key } => {
                if self.remove_map(key) {
                    ActionResponse::Successful
                } else {
                    ActionResponse::Errored(ErrorKind::NotFound)
                }
            }
        }
    }

    fn push_map(&mut self, key: String, data: SomeData) -> Option<SomeData> {
        self.store.insert(key, data)
    }

    fn remove_map(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    fn get_map(&self, key: &str) -> Result<SomeData, ErrorKind> {
        self.store.get(key).cloned().ok_or(ErrorKind::NotFound)
    }
}
