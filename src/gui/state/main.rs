use crate::core::entry::Entry;
use crate::gui::error;
use eframe::egui::Color32;
use std::mem;

pub struct Main {
    service: String,
    username: String,
    password: String,
    notes: String,
    show: bool,
    color: Color32,
    state: State,
}

#[derive(Default)]
enum State {
    New,
    Edit,
    #[default]
    Close,
}

impl Main {
    pub fn service_mut(&mut self) -> &mut String {
        &mut self.service
    }

    pub fn username_mut(&mut self) -> &mut String {
        &mut self.username
    }

    pub fn password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    pub fn notes_mut(&mut self) -> &mut String {
        &mut self.notes
    }

    pub fn show_mut(&mut self) -> &mut bool {
        &mut self.show
    }

    pub fn color(&self) -> Color32 {
        self.color
    }

    pub fn err_color(&mut self) {
        self.color = Color32::RED;
    }

    pub fn common_color(&mut self) {
        self.color = Color32::WHITE;
    }

    pub fn new(&mut self) {
        self.state = State::New;
    }

    pub fn edit(&mut self) {
        self.state = State::Edit;
    }

    pub fn close(&mut self) {
        self.state = State::Close;
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state, State::New) || matches!(self.state, State::Edit)
    }

    pub fn is_empty(&self) -> bool {
        self.service.trim().is_empty()
            || self.username.trim().is_empty()
            || self.password.trim().is_empty()
    }
}

impl Main {
    pub fn take_entry(&mut self) -> error::Result<Entry> {
        if self.is_empty() {
            return Err(error::Error::EmptyData);
        }

        let service = mem::take(&mut self.service).trim().to_owned();
        let username = mem::take(&mut self.username).trim().to_owned();
        let password = mem::take(&mut self.password).trim().to_owned();
        let notes = if self.notes.trim().is_empty() {
            None
        } else {
            Some(mem::take(&mut self.notes).trim().to_owned())
        };

        Ok(Entry::new(service, username, password, notes))
    }
}

impl Default for Main {
    fn default() -> Self {
        Self {
            service: Default::default(),
            username: Default::default(),
            password: Default::default(),
            notes: Default::default(),
            show: false,
            color: Color32::WHITE,
            state: Default::default(),
        }
    }
}
