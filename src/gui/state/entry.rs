use crate::core::entry::Entry;
use crate::core::storage;
use crate::gui::error;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct Entries(Vec<Entry>);

impl Entries {
    pub fn load(&self, key: &[u8]) -> error::Result<Entries> {
        Ok(Entries(storage::load_entries(key)?))
    }

    pub fn save(&mut self, key: &[u8], salt: &[u8]) -> error::Result<()> {
        storage::save_entries(self, key, salt)?;
        Ok(())
    }

    pub fn add(&mut self, entry: Entry, key: &[u8], salt: &[u8]) -> error::Result<()> {
        self.0.push(entry);
        self.save(key, salt)?;
        Ok(())
    }

    pub fn remove(&mut self, idx: usize, key: &[u8], salt: &[u8]) -> error::Result<()> {
        self.0.remove(idx);
        self.save(key, salt)?;
        Ok(())
    }
}

impl Deref for Entries {
    type Target = Vec<Entry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
