use crate::core::error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_lock_secs: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_lock_secs: 300,
        }
    }
}

pub fn settings_path() -> PathBuf {
    super::storage::app_data_dir().join(SETTINGS_FILE)
}

pub fn load() -> AppSettings {
    let path = settings_path();
    if !path.exists() {
        return AppSettings::default();
    }

    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save(settings: &AppSettings) -> error::Result<()> {
    let path = settings_path();
    super::storage::ensure_dir(&super::storage::app_data_dir())?;
    let data = serde_json::to_string_pretty(settings)
        .map_err(|e| error::Error::Storage(format!("Failed to serialize settings: {e}")))?;
    fs::write(&path, data)
        .map_err(|e| error::Error::Storage(format!("Failed to write settings: {e}")))
}
