use crate::error::KpvError;
use crate::storage::Storage;
use std::path::Path;

use super::Execute;

pub struct SaveCommand<'a> {
    pub key: &'a str,
    pub source_path: &'a Path,
}

impl<'a> Execute<()> for SaveCommand<'a> {
    fn execute(&self, storage: &impl Storage) -> Result<(), KpvError> {
        if !self.source_path.exists() || !self.source_path.is_file() {
            return Err(KpvError::EnvFileNotFound);
        }

        storage.save_env(self.key, self.source_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_support::MockStorage;
    use crate::storage::Storage;
    use tempfile::NamedTempFile;

    #[test]
    fn save_invokes_storage_when_source_exists() {
        let file = NamedTempFile::new().expect("failed to create temp file");
        std::fs::write(file.path(), "KEY=value\n").expect("failed to write temp file");

        let command = SaveCommand { key: "my-key", source_path: file.path() };

        let storage = MockStorage::default();
        command.execute(&storage).expect("execution should succeed");

        let calls = storage.save_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "my-key");
        assert_eq!(calls[0].1, file.path().to_path_buf());
    }

    #[test]
    fn save_errors_when_source_missing() {
        let missing_path = std::path::PathBuf::from("/tmp/non-existent-env");
        let command = SaveCommand { key: "missing", source_path: &missing_path };

        let storage = MockStorage::default();
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::EnvFileNotFound)));
        assert!(storage.save_calls.borrow().is_empty());
    }

    #[test]
    fn save_errors_when_source_is_directory() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let command = SaveCommand { key: "dir-key", source_path: dir.path() };

        let storage = MockStorage::default();
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::EnvFileNotFound)));
        assert!(storage.save_calls.borrow().is_empty());
    }

    #[test]
    fn save_propagates_storage_error() {
        struct ErrorStorage;

        impl Storage for ErrorStorage {
            fn save_env(&self, _key: &str, _source_path: &Path) -> Result<(), KpvError> {
                Err(KpvError::from(std::io::Error::other("boom")))
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
                unreachable!()
            }
        }

        let file = NamedTempFile::new().expect("failed to create temp file");
        let command = SaveCommand { key: "err", source_path: file.path() };

        let storage = ErrorStorage;
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::Io(_))));
    }
}
