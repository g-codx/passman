use crate::core::{crypto, storage};
use crate::gui::error;
use crate::gui::state::entry::Entries;
use secrecy::SecretString;
use secrecy::zeroize::{Zeroize, Zeroizing};
use std::mem;

#[derive(Default)]
pub struct Auth {
    key: Zeroizing<[u8; 32]>,
    salt: Zeroizing<[u8; 16]>,
    master: String,
}

impl Auth {
    pub fn unlock(&mut self) -> error::Result<()> {
        let salt = if storage::exists() {
            storage::try_get_salt()?
        } else {
            storage::generate_salt()
        };

        let secret_master = SecretString::from(mem::take(&mut self.master));
        let key = Zeroizing::new(crypto::derive_key(secret_master, &salt)?);
        let salt = Zeroizing::new(salt);

        self.key = key;
        self.salt = salt;
        self.master.zeroize();

        Ok(())
    }

    pub fn verify_master_password(&self, password: SecretString) -> error::Result<()> {
        let derived = crypto::derive_key(password, self.salt.as_slice())?;
        if derived != *self.key {
            return Err(error::Error::InvalidMasterPassword);
        }
        Ok(())
    }

    /// Меняет мастер-пароль и ротирует соль, перешифровывая хранилище.
    pub fn change_master_password(
        &mut self,
        current: SecretString,
        new: SecretString,
        entries: &mut Entries,
    ) -> error::Result<()> {
        self.verify_master_password(current)?;

        let new_salt = storage::generate_salt();
        let new_key = Zeroizing::new(crypto::derive_key(new, &new_salt)?);
        entries.save(new_key.as_slice(), &new_salt)?;

        self.key = new_key;
        self.salt = Zeroizing::new(new_salt);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.master.zeroize();
        self.master.shrink_to_fit();
        self.key.zeroize();
        self.salt.zeroize();
    }
}

impl Auth {
    pub fn key(&self) -> &[u8] {
        self.key.as_slice()
    }

    pub fn salt(&self) -> &[u8] {
        self.salt.as_slice()
    }

    pub fn master_mut(&mut self) -> &mut String {
        &mut self.master
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entry::Entry;
    use crate::core::storage;
    use std::path::PathBuf;
    use std::sync::Mutex;

    static AUTH_TEST_LOCK: Mutex<()> = Mutex::new(());

    struct TempStorage(PathBuf);

    impl TempStorage {
        fn new() -> Self {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join(format!("passman-auth-test-{}.pms", std::process::id()));
            let _ = std::fs::remove_file(&path);
            storage::set_test_storage_path(path.clone());
            Self(path)
        }
    }

    impl Drop for TempStorage {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
            storage::clear_test_storage_path();
        }
    }

    #[test]
    fn change_master_password_rotates_salt() {
        let _lock = AUTH_TEST_LOCK.lock().unwrap();
        let _guard = TempStorage::new();

        let mut auth = Auth::default();
        let old_salt = [1u8; 16];
        let old_key = Zeroizing::new(
            crypto::derive_key(SecretString::new("old-pass".into()), &old_salt).unwrap(),
        );
        auth.key = old_key;
        auth.salt = Zeroizing::new(old_salt);

        let mut entries = Entries::default();
        entries
            .add(
                Entry::new(
                    "svc".into(),
                    "user".into(),
                    SecretString::new("pw".into()),
                    None,
                ),
                auth.key(),
                auth.salt(),
            )
            .unwrap();

        auth.change_master_password(
            SecretString::new("old-pass".into()),
            SecretString::new("new-pass".into()),
            &mut entries,
        )
        .unwrap();

        assert_ne!(auth.salt(), &old_salt);
        assert!(
            auth.verify_master_password(SecretString::new("new-pass".into()))
                .is_ok()
        );
        assert!(
            auth.verify_master_password(SecretString::new("old-pass".into()))
                .is_err()
        );
    }
}
