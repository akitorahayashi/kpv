use crate::error::KpvError;
use crate::storage::Storage;

use super::Execute;

pub struct ListCommand;

impl Execute<Vec<String>> for ListCommand {
    fn execute(&self, storage: &impl Storage) -> Result<Vec<String>, KpvError> {
        let mut keys = storage.list_keys()?;
        keys.sort();
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_support::MockStorage;
    use crate::storage::Storage;

    #[test]
    fn list_returns_sorted_keys() {
        let storage = MockStorage::default();
        storage.set_list_keys(vec!["beta", "alpha", "gamma"]);

        let command = ListCommand;
        let keys = command.execute(&storage).expect("list should succeed");
        assert_eq!(keys, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn list_propagates_storage_error() {
        struct ErrorStorage;

        impl Storage for ErrorStorage {
            fn save_env(&self, _key: &str, _source_path: &std::path::Path) -> Result<(), KpvError> {
                unreachable!()
            }

            fn link_env(&self, _key: &str, _dest_path: &std::path::Path) -> Result<(), KpvError> {
                unreachable!()
            }

            fn list_keys(&self) -> Result<Vec<String>, KpvError> {
                Err(KpvError::from(std::io::Error::other("list failure")))
            }

            fn get_key_env_path(&self, _key: &str) -> std::path::PathBuf {
                std::path::PathBuf::new()
            }

            fn check_key_exists(&self, _key: &str) -> bool {
                false
            }

            fn delete_env(&self, _key: &str) -> Result<(), KpvError> {
                unreachable!()
            }
        }

        let command = ListCommand;
        let storage = ErrorStorage;
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::Io(_))));
    }
}
