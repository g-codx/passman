use crate::core::entry::Entry;
use crate::gui::error;
use eframe::egui::Color32;
use std::mem;

pub struct Main {
    pub idx: Option<usize>,
    pub service: String,
    pub username: String,
    pub password: String,
    pub notes: String,
    pub show: bool,
    pub color: Color32,
    pub editor_state: EditorState,
    pub search_line: String,
}

#[derive(Default)]
pub enum EditorState {
    New,
    Edit,
    #[default]
    Close,
}

impl Main {
    pub fn err_color(&mut self) {
        self.color = Color32::RED;
    }

    pub fn common_color(&mut self) {
        self.color = Color32::WHITE;
    }

    pub fn is_open(&self) -> bool {
        matches!(self.editor_state, EditorState::New)
            || matches!(self.editor_state, EditorState::Edit)
    }

    pub fn is_empty(&self) -> bool {
        self.service.trim().is_empty()
            || self.username.trim().is_empty()
            || self.password.trim().is_empty()
    }
}

impl Main {
    pub fn take_edit_data(&mut self) -> error::Result<(Option<usize>, Entry)> {
        if self.is_empty() {
            return Err(error::Error::EmptyData);
        }

        let idx = mem::take(&mut self.idx);
        let service = mem::take(&mut self.service).trim().to_owned();
        let username = mem::take(&mut self.username).trim().to_owned();
        let password = mem::take(&mut self.password).trim().to_owned();
        let notes = if self.notes.trim().is_empty() {
            None
        } else {
            Some(mem::take(&mut self.notes).trim().to_owned())
        };

        Ok((idx, Entry::new(service, username, password, notes)))
    }
}

impl Default for Main {
    fn default() -> Self {
        Self {
            idx: None,
            service: Default::default(),
            username: Default::default(),
            password: Default::default(),
            notes: Default::default(),
            show: false,
            color: Color32::WHITE,
            editor_state: Default::default(),
            search_line: "".to_string(),
        }
    }
}
