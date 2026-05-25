use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone)]
pub struct Entry {
    pub uuid: String,
    pub service: String,
    pub username: String,
    pub password: SecretString,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct EntrySerde {
    uuid: String,
    service: String,
    username: String,
    password: String,
    notes: Option<String>,
}

impl Serialize for Entry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        EntrySerde {
            uuid: self.uuid.clone(),
            service: self.service.clone(),
            username: self.username.clone(),
            password: self.password.expose_secret().to_owned(),
            notes: self.notes.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = EntrySerde::deserialize(deserializer)?;
        Ok(Entry {
            uuid: raw.uuid,
            service: raw.service,
            username: raw.username,
            password: SecretString::new(raw.password.into_boxed_str()),
            notes: raw.notes,
        })
    }
}

impl Entry {
    pub fn new(
        service: String,
        username: String,
        password: impl Into<SecretString>,
        notes: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            service,
            username,
            password: password.into(),
            notes,
        }
    }

    pub fn password(&self) -> &str {
        self.password.expose_secret()
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("uuid", &self.uuid)
            .field("service", &self.service)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("notes", &self.notes)
            .finish()
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
                password: [redacted]\n\
                notes: {}\n",
            self.uuid,
            self.service,
            self.username,
            self.notes.as_deref().unwrap_or("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_redacts_password() {
        let entry = Entry::new(
            "svc".into(),
            "user".into(),
            SecretString::new("secret123".into()),
            None,
        );
        let output = format!("{entry}");
        assert!(!output.contains("secret123"));
        assert!(output.contains("[redacted]"));
    }

    #[test]
    fn serde_roundtrip() {
        let entries = vec![Entry::new(
            "svc".into(),
            "user".into(),
            SecretString::new("p@ss".into()),
            Some("note".into()),
        )];
        let json = serde_json::to_vec(&entries).unwrap();
        let restored: Vec<Entry> = serde_json::from_slice(&json).unwrap();
        assert_eq!(restored[0].password(), "p@ss");
    }
}
