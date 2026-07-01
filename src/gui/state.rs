mod auth;
mod command;
mod entry;
mod main;
mod security;
mod session;
mod status;

pub use command::{Cmd, Command};
pub use main::EditorState;

use crate::core::crypto;
use crate::core::storage;
use crate::gui::error;
use crate::gui::state::auth::Auth;
use crate::gui::state::entry::Entries;
use crate::gui::state::main::Main;
use crate::gui::state::security::Security;
use crate::gui::state::session::Session;
use crate::gui::state::status::Status;
use eframe::egui;
use secrecy::SecretString;
use std::mem;
use std::time::Duration;

#[derive(Default)]
pub struct State {
    auth: Auth,
    status: Status,
    entries: Entries,
    cmd: Command,
    session: Session,
    security: Security,

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
                self.session.touch();
                self.security.auto_lock_secs = self.session.auto_lock_secs();
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

    pub fn lock_session(&mut self) {
        self.auth.clear();
        self.entries.clear();
        self.main.reset_editor();
        self.security.clear_password_fields();
        self.security.change_password_open = false;
        self.security.settings_open = false;
        self.session.last_activity = None;
        self.status.lock();
    }

    pub fn register_activity(&mut self, ctx: &egui::Context) {
        if self.is_locked() {
            return;
        }

        let active = ctx.input(|input| {
            !input.events.is_empty()
                || input.pointer.any_pressed()
                || input.pointer.any_click()
                || input.pointer.any_released()
                || input.smooth_scroll_delta.length_sq() > 0.0
        });

        if active {
            self.session.touch();
        }

        if self.session.should_lock() {
            self.lock_session();
        }
    }

    pub fn schedule_idle_check(&self, ctx: &egui::Context) {
        if !self.is_locked() {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    pub fn backup(&mut self) {
        match storage::backup_storage() {
            Ok(path) => self
                .status
                .message(format!("Backup saved: {}", path.display())),
            Err(e) => self.status.error(e.to_string()),
        }
    }

    pub fn export(&mut self) {
        match storage::export_entries_json(self.entries.as_slice()) {
            Ok(path) => self
                .status
                .message(format!("Export saved: {}", path.display())),
            Err(e) => self.status.error(e.to_string()),
        }
    }

    pub fn change_master_password(&mut self) -> error::Result<()> {
        if self.security.new != self.security.confirm {
            return Err(error::Error::PasswordMismatch);
        }
        if self.security.current.trim().is_empty()
            || self.security.new.trim().is_empty()
        {
            return Err(error::Error::EmptyData);
        }

        let current = SecretString::new(self.security.current.as_str().into());
        let new = SecretString::new(self.security.new.as_str().into());

        self.auth
            .change_master_password(current, new, &mut self.entries)?;
        self.security.clear_password_fields();
        self.security.change_password_open = false;
        self.status
            .message("Master password changed, salt rotated".to_string());
        Ok(())
    }

    pub fn save_settings(&mut self) {
        self.security.auto_lock_secs = self.security.auto_lock_secs.clamp(60, 3600);
        match self
            .session
            .set_auto_lock_secs(self.security.auto_lock_secs)
        {
            Ok(()) => {
                self.security.settings_open = false;
                self.status.message(format!(
                    "Auto-lock timeout: {} min",
                    self.session.auto_lock_secs() / 60
                ));
            }
            Err(e) => self.status.error(e.to_string()),
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

    pub fn security_mut(&mut self) -> &mut Security {
        &mut self.security
    }

    pub fn auto_lock_secs(&self) -> u64 {
        self.session.auto_lock_secs()
    }

    pub fn show_error(&mut self, msg: String) {
        self.status.error(msg);
    }

    pub fn edit_data(&mut self, idx: usize) {
        let entry = self.entries[idx].clone();

        self.main.idx = Some(idx);
        self.main.password = entry.password().to_owned();
        self.main.service = entry.service;
        self.main.username = entry.username;
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
