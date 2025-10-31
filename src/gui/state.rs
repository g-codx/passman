mod auth;
mod command;
mod entry;
mod main;
mod status;

pub use command::{Cmd, Command};
pub use main::EditorState;

use crate::core::crypto;
use crate::gui::error;
use crate::gui::state::auth::Auth;
use crate::gui::state::entry::Entries;
use crate::gui::state::main::Main;
use crate::gui::state::status::Status;
use eframe::egui;
use std::mem;

#[derive(Default)]
pub struct State {
    auth: Auth,
    status: Status,
    entries: Entries,
    cmd: Command,

    pub main: Main,
}

impl State {
    pub fn load(&mut self) {
        match self
            .auth
            .unlock()
            .and_then(|_| self.entries.load(self.auth.key()))
        {
            Ok(loaded_entries) => {
                self.entries = loaded_entries;
                self.status.unlock();
            }
            Err(error::Error::Core(crate::core::error::Error::AesGcm(_))) => {
                self.auth.clear();
                self.status.error("Auth failed".to_string());
            }
            Err(e) => {
                self.auth.clear();
                self.status.error(e.to_string());
            }
        }
    }

    pub fn add(&mut self) {
        if let Err(err) = self
            .main
            .take_edit_data()
            .and_then(|(_, e)| self.entries.add(e, self.auth.key(), self.auth.salt()))
        {
            self.status.error(err.to_string());
            self.main.err_color();
        } else {
            save(&self.auth, &mut self.entries, &mut self.status);
            self.main.common_color();
        }
    }

    pub fn update(&mut self) {
        if let Err(err) = self.main.take_edit_data().map(|(idx, e)| {
            if let Some(idx) = idx {
                let _ = mem::replace(&mut self.entries[idx], e);
                save(&self.auth, &mut self.entries, &mut self.status);
            }
        }) {
            self.status.error(err.to_string());
            self.main.err_color();
        }
    }

    pub fn remove(&mut self, idx: usize) {
        if let Err(err) = self.entries.remove(idx, self.auth.key(), self.auth.salt()) {
            self.status.error(err.to_string());
        } else {
            save(&self.auth, &mut self.entries, &mut self.status);
            self.status.message("Data deleted".to_string());
        }
    }

    pub fn generate_password(&self) -> String {
        crypto::generate_password(32)
    }

    pub fn take_cmd(&mut self) -> Option<Cmd> {
        self.cmd.take()
    }

    pub fn set_cmd(&mut self, cmd: Cmd) {
        self.cmd.set(cmd);
    }
}

impl State {
    pub fn master_mut(&mut self) -> &mut String {
        self.auth.master_mut()
    }

    pub fn is_locked(&self) -> bool {
        self.status.is_locked()
    }

    pub fn status(&self) -> String {
        self.status.status()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn list(&self) -> &Entries {
        &self.entries
    }

    pub fn text_color(&self) -> egui::Color32 {
        self.status.color()
    }

    pub fn edit_data(&mut self, idx: usize) {
        let entry = self.entries[idx].clone();

        self.main.idx = Some(idx);
        self.main.service = entry.service;
        self.main.username = entry.username;
        self.main.password = entry.password;
        self.main.notes = entry.notes.unwrap_or_default();
    }
}

fn save(auth: &Auth, entries: &mut Entries, status: &mut Status) {
    if let Err(err) = entries.save(auth.key(), auth.salt()) {
        status.error(err.to_string());
    } else {
        status.message("The storage has been updated".to_string());
    }
}
