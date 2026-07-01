use crate::core::settings::{self, AppSettings};
use crate::gui::error;
use std::time::Instant;

pub struct Session {
    pub last_activity: Option<Instant>,
    auto_lock_secs: u64,
}

impl Session {
    pub fn new() -> Self {
        let settings = settings::load();
        Self {
            last_activity: None,
            auto_lock_secs: settings.auto_lock_secs,
        }
    }

    pub fn touch(&mut self) {
        self.last_activity = Some(Instant::now());
    }

    pub fn auto_lock_secs(&self) -> u64 {
        self.auto_lock_secs
    }

    pub fn set_auto_lock_secs(&mut self, secs: u64) -> error::Result<()> {
        self.auto_lock_secs = secs.clamp(60, 3600);
        settings::save(&AppSettings {
            auto_lock_secs: self.auto_lock_secs,
        })?;
        Ok(())
    }

    pub fn should_lock(&self) -> bool {
        let Some(last) = self.last_activity else {
            return false;
        };
        last.elapsed().as_secs() >= self.auto_lock_secs
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
