// data_store.rs
use std::{collections::HashMap, io::ErrorKind, time::Instant};

use crate::parser::Action;

#[derive(Clone, Debug)]
pub struct SomeData {
    pub value: String,
    pub expires_at: Option<Instant>,
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

    pub fn perform_action(&mut self, action: Action) -> ActionResponse {
        match action {
            Action::Set {
                key,
                value,
                expires_at,
            } => {
                self.push_map(
                    key.to_string(),
                    SomeData {
                        value: value.clone(),
                        expires_at: expires_at,
                    },
                );
                ActionResponse::Successful
            }
            Action::Get { key } => match self.get_map(&key.to_string()) {
                Ok(data) => ActionResponse::NoteFound(data),
                Err(e) => ActionResponse::Errored(e),
            },
            Action::Delete { key } => {
                if self.remove_map(&key.to_string()) {
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

    fn get_map(&mut self, key: &str) -> Result<SomeData, ErrorKind> {
        let s = self.store.get(key).cloned().ok_or(ErrorKind::NotFound);
        match s {
            Ok(data) => match data.expires_at {
                Some(time) => {
                    if Instant::now() > time {
                        self.store.remove(key);
                        return Err(ErrorKind::NotFound);
                    } else {
                        return Ok(data);
                    }
                }
                None => return Ok(data),
            },
            Err(_) => return Err(ErrorKind::NotFound),
        }
    }
}
