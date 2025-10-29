use crate::core::{crypto, storage};
use crate::gui::error;
use secrecy::SecretString;
use secrecy::zeroize::{Zeroize, Zeroizing};
use std::mem;
use std::path::Path;

#[derive(Default)]
pub struct Auth {
    key: Zeroizing<[u8; 32]>,
    salt: Zeroizing<[u8; 16]>,
    master: String,
}

impl Auth {
    pub fn unlock(&mut self) -> error::Result<()> {
        let salt = if Path::new(crate::core::FILE).exists() {
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
