use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Entry {
    uuid: String,
    pub service: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}

impl Entry {
    pub fn new(service: String, username: String, password: String, notes: Option<String>) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            service,
            username,
            password,
            notes,
        }
    }
    
    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "***\n\
                uuid: {}\n\
                service: {}\n\
                username: {}\n\
                password: {}\n\
                notes: {}\n",
            self.uuid,
            self.service,
            self.username,
            self.password,
            self.notes.as_ref().unwrap_or(&String::default())
        )
    }
}
