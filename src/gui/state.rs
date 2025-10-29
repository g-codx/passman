mod auth;
mod command;
mod entry;
mod main;
mod status;

use crate::core::crypto;
use crate::gui::state::auth::Auth;
use crate::gui::state::entry::Entries;
use crate::gui::state::main::Main;
use crate::gui::state::status::Status;
use eframe::egui;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Default)]
pub struct State {
    auth: RefCell<Auth>,
    status: RefCell<Status>,
    entries: RefCell<Entries>,
    main: RefCell<Main>,
}

impl State {
    pub fn load(&self) {
        let auth = &mut *self.auth.borrow_mut();
        let entries = &mut *self.entries.borrow_mut();
        let status = &mut *self.status.borrow_mut();

        match auth.unlock().and_then(|_| entries.load(auth.key())) {
            Ok(loaded_entries) => {
                *entries = loaded_entries;
                status.unlock();
            }
            Err(err) => {
                auth.clear();
                status.lock();
                status.error(err.to_string());
            }
        }
    }

    pub fn add(&self, main: &mut Main) {
        let auth = &mut *self.auth.borrow_mut();
        let entries = &mut *self.entries.borrow_mut();
        let status = &mut *self.status.borrow_mut();
        //todo!
        // let main = &mut *self.main.borrow_mut();

        if let Err(err) = main
            .take_entry()
            .and_then(|e| entries.add(e, auth.key(), auth.salt()))
        {
            status.error(err.to_string());
            main.err_color();
        } else {
            save(auth, entries, status);
            main.common_color();
        }
    }

    pub fn remove(&self, uuid: &str) {
        let auth = &*self.auth.borrow();
        let entries = &mut *self.entries.borrow_mut();
        let status = &mut *self.status.borrow_mut();

        if let Err(err) = entries.remove(uuid, auth.key(), auth.salt()) {
            status.error(err.to_string());
        } else {
            save(auth, entries, status);
            status.message("Data deleted".to_string());
        }
    }

    pub fn new_entry(&self) {
        self.main.borrow_mut().new();
    }

    pub fn edit_entry(&self) {
        self.main.borrow_mut().edit();
    }

    pub fn generate_password(&self) -> String {
        crypto::generate_password(32)
    }
}

impl State {
    pub fn is_locked(&self) -> bool {
        self.status.borrow().is_locked()
    }

    pub fn status(&self) -> String {
        self.status.borrow().status()
    }

    pub fn len(&self) -> usize {
        self.entries.borrow().len()
    }

    pub fn list(&self) -> Ref<'_, Entries> {
        self.entries.borrow()
    }

    pub fn master(&mut self) -> &mut String {
        self.auth.get_mut().master_mut()
    }

    pub fn main(&self) -> RefMut<'_, Main> {
        self.main.borrow_mut()
    }

    pub fn text_color(&self) -> egui::Color32 {
        self.status.borrow().color()
    }

    pub fn edit_data(&mut self, idx: usize) {
        let entry = self.entries.borrow()[idx].clone();
        let main = &mut *self.main();

        *main.service_mut() = entry.service;
        *main.username_mut() = entry.username;
        *main.password_mut() = entry.password;
        *main.notes_mut() = entry.notes.unwrap_or_default();
    }
}

fn save(auth: &Auth, entries: &mut Entries, status: &mut Status) {
    if let Err(err) = entries.save(auth.key(), auth.salt()) {
        status.error(err.to_string());
    } else {
        status.message("The storage has been updated".to_string());
    }
}
