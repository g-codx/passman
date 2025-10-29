use eframe::egui::Color32;

#[derive(Default)]
pub struct Status {
    is_unlocked: bool,
    message: StatusMessage,
    color: Color32,
}

impl Status {
    pub fn is_locked(&self) -> bool {
        !self.is_unlocked
    }

    pub fn status(&self) -> String {
        match &self.message {
            StatusMessage::Locked => "🔒 Locked".to_string(),
            StatusMessage::Unlocked => "🔓 Unlocked".to_string(),
            StatusMessage::Error(e) => format!("❎ : {e}"),
            StatusMessage::Message(m) => format!("✅ : {m}"),
        }
    }

    pub fn lock(&mut self) {
        self.is_unlocked = false;
        self.message = StatusMessage::Locked;
        self.color = Color32::RED;
    }

    pub fn unlock(&mut self) {
        self.is_unlocked = true;
        self.message = StatusMessage::Unlocked;
        self.color = Color32::GREEN;
    }

    pub fn error(&mut self, err: String) {
        self.message = StatusMessage::Error(err);
        self.color = Color32::RED;
    }

    pub fn message(&mut self, msg: String) {
        self.message = StatusMessage::Message(msg);
        self.color = Color32::GREEN;
    }

    pub fn color(&self) -> Color32 {
        self.color
    }
}

#[derive(Default)]
enum StatusMessage {
    #[default]
    Locked,
    Unlocked,
    Error(String),
    Message(String),
}
