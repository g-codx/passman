use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub uuid: String,
    pub service: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
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
