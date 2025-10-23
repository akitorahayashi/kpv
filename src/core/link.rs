use crate::error::KpvError;
use crate::storage::Storage;
use std::path::Path;

use super::Execute;

pub struct LinkCommand<'a> {
    pub key: &'a str,
    pub dest_path: &'a Path,
}

impl<'a> Execute<()> for LinkCommand<'a> {
    fn execute(&self, storage: &impl Storage) -> Result<(), KpvError> {
        if !storage.check_key_exists(self.key) {
            return Err(KpvError::key_not_found(self.key));
        }

        if self.dest_path.symlink_metadata().is_ok() {
            return Err(KpvError::EnvAlreadyExists);
        }

        storage.link_env(self.key, self.dest_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_support::MockStorage;
    use crate::storage::Storage;

    #[test]
    fn link_invokes_storage_when_key_exists_and_dest_free() {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let dest = temp_dir.path().join(".env");

        let storage = MockStorage::with_existing_keys(&["demo-key"]);
        let command = LinkCommand { key: "demo-key", dest_path: &dest };

        command.execute(&storage).expect("link should succeed");

        let calls = storage.link_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "demo-key");
        assert_eq!(calls[0].1, dest);
    }

    #[test]
    fn link_errors_when_key_missing() {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let dest = temp_dir.path().join(".env");
        let storage = MockStorage::default();

        let command = LinkCommand { key: "absent", dest_path: &dest };

        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::KeyNotFound(key)) if key == "absent"));
        assert!(storage.link_calls.borrow().is_empty());
    }

    #[test]
    fn link_errors_when_dest_exists() {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let dest = temp_dir.path().join(".env");
        std::fs::write(&dest, "placeholder").expect("failed to create existing dest");

        let storage = MockStorage::with_existing_keys(&["demo-key"]);
        let command = LinkCommand { key: "demo-key", dest_path: &dest };

        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::EnvAlreadyExists)));
        assert!(storage.link_calls.borrow().is_empty());
    }

    #[test]
    fn link_propagates_storage_error() {
        use std::path::{Path, PathBuf};

        struct ErrorStorage;

        impl Storage for ErrorStorage {
            fn save_env(&self, _key: &str, _source_path: &Path) -> Result<(), KpvError> {
                unreachable!()
            }

            fn link_env(&self, _key: &str, _dest_path: &Path) -> Result<(), KpvError> {
                Err(KpvError::from(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "nope",
                )))
            }

            fn list_keys(&self) -> Result<Vec<String>, KpvError> {
                unreachable!()
            }

            fn get_key_env_path(&self, _key: &str) -> PathBuf {
                PathBuf::new()
            }

            fn check_key_exists(&self, _key: &str) -> bool {
                true
            }

            fn delete_env(&self, _key: &str) -> Result<(), KpvError> {
                unreachable!()
            }
        }

        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let dest = temp_dir.path().join(".env");
        let command = LinkCommand { key: "demo-key", dest_path: &dest };

        let storage = ErrorStorage;
        let result = command.execute(&storage);
        assert!(matches!(result, Err(KpvError::Io(_))));
    }
}
