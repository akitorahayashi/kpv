use crate::error::KpvError;
use crate::storage::Storage;

use super::Execute;

pub struct DeleteCommand<'a> {
    pub key: &'a str,
}

impl<'a> Execute<()> for DeleteCommand<'a> {
    fn execute(&self, storage: &impl Storage) -> Result<(), KpvError> {
        storage.delete_env(self.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_support::MockStorage;

    #[test]
    fn delete_invokes_storage() {
        let command = DeleteCommand { key: "test-key" };
        let storage = MockStorage::default();

        command.execute(&storage).expect("execution should succeed");

        let calls = storage.delete_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], "test-key");
    }

    #[test]
    fn delete_propagates_storage_error() {
        use std::path::Path;

        struct ErrorStorage;

        impl Storage for ErrorStorage {
            fn save_env(&self, _key: &str, _source_path: &Path) -> Result<(), KpvError> {
                unreachable!()
            }

            fn link_env(&self, _key: &str, _dest_path: &Path) -> Result<(), KpvError> {
                unreachable!()
            }

            fn list_keys(&self) -> Result<Vec<String>, KpvError> {
                unreachable!()
            }

            fn get_key_env_path(&self, _key: &str) -> std::path::PathBuf {
                unreachable!()
            }

            fn check_key_exists(&self, _key: &str) -> bool {
                unreachable!()
            }

            fn delete_env(&self, _key: &str) -> Result<(), KpvError> {
                Err(KpvError::from(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "permission denied",
                )))
            }
        }

        let command = DeleteCommand { key: "protected" };
        let storage = ErrorStorage;
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::Io(_))));
    }
}
