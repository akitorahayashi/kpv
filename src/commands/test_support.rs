use crate::error::KpvError;
use crate::storage::Storage;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct MockStorage {
    pub save_calls: RefCell<Vec<(String, PathBuf)>>,
    pub link_calls: RefCell<Vec<(String, PathBuf)>>,
    pub delete_calls: RefCell<Vec<String>>,
    pub list_keys_values: RefCell<Vec<String>>,
    existing_keys: RefCell<HashSet<String>>,
}

impl MockStorage {
    pub fn with_existing_keys(keys: &[&str]) -> Self {
        let storage = Self::default();
        storage.set_existing_keys(keys);
        storage
    }

    pub fn set_existing_keys(&self, keys: &[&str]) {
        let mut set = self.existing_keys.borrow_mut();
        set.clear();
        for key in keys {
            set.insert((*key).to_string());
        }
    }

    pub fn set_list_keys<I>(&self, keys: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut values = self.list_keys_values.borrow_mut();
        values.clear();
        values.extend(keys.into_iter().map(Into::into));
    }
}

impl Storage for MockStorage {
    fn save_env(&self, key: &str, source_path: &Path) -> Result<(), KpvError> {
        self.save_calls.borrow_mut().push((key.to_string(), source_path.to_path_buf()));
        Ok(())
    }

    fn link_env(&self, key: &str, dest_path: &Path) -> Result<(), KpvError> {
        self.link_calls.borrow_mut().push((key.to_string(), dest_path.to_path_buf()));
        Ok(())
    }

    fn list_keys(&self) -> Result<Vec<String>, KpvError> {
        Ok(self.list_keys_values.borrow().clone())
    }

    fn get_key_env_path(&self, key: &str) -> PathBuf {
        PathBuf::from("/mock").join(key).join(".env")
    }

    fn check_key_exists(&self, key: &str) -> bool {
        self.existing_keys.borrow().contains(key)
    }

    fn delete_env(&self, key: &str) -> Result<(), KpvError> {
        self.delete_calls.borrow_mut().push(key.to_string());
        Ok(())
    }
}
