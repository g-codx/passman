use crate::core::entry::Entry;
use crate::core::{crypto, error};
use aes_gcm::aead::OsRng;
use aes_gcm::aead::rand_core::RngCore;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
#[cfg(test)]
use std::sync::RwLock;

pub const FILE_NAME: &str = "passman.pms";

const APP_DIR: &str = "passman";
const LEGACY_FILE: &str = "passman.pms";

const SALT_LEN: usize = 16;

static MIGRATED: OnceLock<()> = OnceLock::new();

#[cfg(test)]
static TEST_STORAGE_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Абсолютный путь к файлу хранилища (не зависит от cwd).
///
/// Linux: `~/.local/share/passman/passman.pms`
/// macOS: `~/Library/Application Support/passman/passman.pms`
/// Windows: `%APPDATA%\passman\passman.pms`
pub fn storage_path() -> PathBuf {
    #[cfg(test)]
    if let Some(path) = TEST_STORAGE_PATH.read().expect("test storage lock").clone() {
        return path;
    }

    MIGRATED.get_or_init(migrate_legacy_storage);
    default_storage_path()
}

pub fn exists() -> bool {
    storage_path().exists()
}

pub fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .map(|dir| dir.join(APP_DIR))
        .unwrap_or_else(|| PathBuf::from(APP_DIR))
}

fn default_storage_path() -> PathBuf {
    app_data_dir().join(FILE_NAME)
}

fn migrate_legacy_storage() {
    let target = default_storage_path();
    let legacy = PathBuf::from(LEGACY_FILE);

    if target.exists() || !legacy.exists() {
        return;
    }

    if let Some(parent) = target.parent() {
        if fs::create_dir_all(parent).is_err() {
            return;
        }
    }

    let _ = fs::rename(&legacy, &target);
}

pub fn ensure_dir(path: &Path) -> error::Result<()> {
    if path.as_os_str().is_empty() {
        return Ok(());
    }

    fs::create_dir_all(path)
        .map_err(|e| error::Error::Storage(format!("Failed to create directory: {e}")))
}

fn ensure_storage_dir(path: &Path) -> error::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    if parent.as_os_str().is_empty() {
        return Ok(());
    }

    ensure_dir(parent)
}

fn timestamp_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs();
    format!("{secs}")
}

/// Копирует зашифрованное хранилище в `…/passman/backups/`.
pub fn backup_storage() -> error::Result<PathBuf> {
    if !exists() {
        return Err(error::Error::Storage("Nothing to backup: storage file is missing".into()));
    }

    let backup_dir = app_data_dir().join("backups");
    ensure_dir(&backup_dir)?;

    let backup_path = backup_dir.join(format!("passman-{suffix}.pms", suffix = timestamp_suffix()));
    fs::copy(storage_path(), &backup_path)
        .map_err(|e| error::Error::Storage(format!("Failed to create backup: {e}")))?;

    Ok(backup_path)
}

/// Экспортирует записи в JSON (пароли в открытом виде).
pub fn export_entries_json(entries: &[Entry]) -> error::Result<PathBuf> {
    let export_dir = app_data_dir().join("exports");
    ensure_dir(&export_dir)?;

    let export_path = export_dir.join(format!(
        "passman-export-{suffix}.json",
        suffix = timestamp_suffix()
    ));
    let data = serde_json::to_vec_pretty(entries)
        .map_err(|e| error::Error::Storage(format!("Failed to export entries: {e}")))?;
    fs::write(&export_path, data)
        .map_err(|e| error::Error::Storage(format!("Failed to write export: {e}")))?;

    Ok(export_path)
}

/// Загружает записи из зашифрованного файла хранилища.
pub fn load_entries(key: &[u8]) -> error::Result<Vec<Entry>> {
    if !exists() {
        return Ok(vec![]);
    }
    let data = read_storage_file()?;
    let (_salt, encrypted) = split_storage(data.as_slice())?;
    let decrypted = crypto::decrypt_aes256gcm(key, encrypted)?;
    serde_json::from_slice(&decrypted)
        .map_err(|e| error::Error::Storage(format!("Failed to read storage; {e:?}")))
}

/// Сохраняет список записей в зашифрованном файле.
pub fn save_entries(entries: &[Entry], key: &[u8], salt: &[u8]) -> error::Result<()> {
    let path = storage_path();
    ensure_storage_dir(&path)?;

    let plaintext = serde_json::to_vec(entries)
        .map_err(|e| error::Error::Storage(format!("Failed to save storage; {e:?}")))?;
    let mut ciphertext = crypto::encrypt_aes256gcm(key, &plaintext)?;
    let mut result = salt.to_vec();
    result.append(&mut ciphertext);
    fs::write(&path, result)
        .map_err(|e| error::Error::Storage(format!("Failed to save storage: {e}")))
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub fn try_get_salt() -> error::Result<[u8; 16]> {
    let data = read_storage_file()?;
    let (salt, _) = split_storage(data.as_slice())?;
    Ok(salt)
}

fn read_storage_file() -> error::Result<Vec<u8>> {
    let path = storage_path();
    fs::read(&path).map_err(|e| error::Error::Storage(format!("Failed to read storage: {e}")))
}

fn split_storage(data: &[u8]) -> error::Result<([u8; SALT_LEN], &[u8])> {
    if data.len() < SALT_LEN {
        return Err(error::Error::Storage(format!(
            "Storage file too short: expected at least {SALT_LEN} bytes, got {}",
            data.len()
        )));
    }

    let mut salt = [0u8; SALT_LEN];
    salt.copy_from_slice(&data[..SALT_LEN]);
    Ok((salt, &data[SALT_LEN..]))
}

#[cfg(any(test, doctest))]
pub(crate) fn set_test_storage_path(path: PathBuf) {
    *TEST_STORAGE_PATH
        .write()
        .expect("test storage lock") = Some(path);
}

#[cfg(any(test, doctest))]
pub(crate) fn clear_test_storage_path() {
    *TEST_STORAGE_PATH
        .write()
        .expect("test storage lock") = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    static STORAGE_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct TempStorage(PathBuf);

    impl TempStorage {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "passman-storage-test-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("clock")
                    .as_nanos()
            ));
            let _ = fs::remove_file(&path);
            set_test_storage_path(path.clone());
            Self(path)
        }
    }

    impl Drop for TempStorage {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
            clear_test_storage_path();
        }
    }

    #[test]
    fn storage_path_is_absolute() {
        let _lock = STORAGE_TEST_LOCK.lock().unwrap();
        let _guard = TempStorage::new();
        assert!(storage_path().is_absolute());
    }

    #[test]
    fn try_get_salt_rejects_short_file() {
        let _lock = STORAGE_TEST_LOCK.lock().unwrap();
        let _guard = TempStorage::new();
        let path = storage_path();
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(&[1, 2, 3]).unwrap();

        let err = try_get_salt().unwrap_err();
        assert!(matches!(err, error::Error::Storage(_)));
        assert!(err.to_string().contains("too short"));
    }

    #[test]
    fn load_entries_rejects_short_file() {
        let _lock = STORAGE_TEST_LOCK.lock().unwrap();
        let _guard = TempStorage::new();
        fs::write(storage_path(), [0u8; 8]).unwrap();

        let key = [0u8; 32];
        let err = load_entries(&key).unwrap_err();
        assert!(matches!(err, error::Error::Storage(_)));
        assert!(err.to_string().contains("too short"));
    }

    #[test]
    fn try_get_salt_reads_valid_header() {
        let _lock = STORAGE_TEST_LOCK.lock().unwrap();
        let _guard = TempStorage::new();
        let salt = [7u8; SALT_LEN];
        fs::write(storage_path(), salt).unwrap();

        assert_eq!(try_get_salt().unwrap(), salt);
    }

    #[test]
    fn save_entries_creates_parent_directory() {
        let _lock = STORAGE_TEST_LOCK.lock().unwrap();
        let path = std::env::temp_dir().join(format!(
            "passman-dir-test-{}",
            std::process::id()
        ));
        let file = path.join("nested").join(FILE_NAME);
        set_test_storage_path(file.clone());

        let entries = vec![Entry::new(
            "svc".into(),
            "user".into(),
            secrecy::SecretString::new("pw".into()),
            None,
        )];
        let key = [1u8; 32];
        let salt = [2u8; 16];

        save_entries(&entries, &key, &salt).unwrap();
        assert!(file.is_file());

        let _ = fs::remove_dir_all(path);
        clear_test_storage_path();
    }
}
