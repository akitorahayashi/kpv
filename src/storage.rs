use crate::error::KpvError;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) trait Storage {
    fn save_env(&self, key: &str, source_path: &Path) -> Result<(), KpvError>;
    fn link_env(&self, key: &str, dest_path: &Path) -> Result<(), KpvError>;
    fn list_keys(&self) -> Result<Vec<String>, KpvError>;
    fn get_key_env_path(&self, key: &str) -> PathBuf;
    fn check_key_exists(&self, key: &str) -> bool;
    fn delete_env(&self, key: &str) -> Result<(), KpvError>;
}

#[derive(Debug, Clone)]
pub(crate) struct FilesystemStorage {
    root_path: PathBuf,
}

impl FilesystemStorage {
    pub fn new_default() -> Result<Self, KpvError> {
        let home = std::env::var("HOME").map_err(|_| KpvError::HomeNotConfigured)?;
        Ok(Self { root_path: PathBuf::from(home).join(".config").join("kpv") })
    }

    fn is_key_valid(key: &str) -> bool {
        use std::path::{Component, Path};
        !key.is_empty()
            && key.chars().all(|c| c.is_alphanumeric() || c == '-')
            && Path::new(key).components().all(|c| matches!(c, Component::Normal(_)))
    }

    fn ensure_valid_key(&self, key: &str) -> Result<(), KpvError> {
        if Self::is_key_valid(key) {
            Ok(())
        } else {
            Err(KpvError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid key: {key}"),
            )))
        }
    }

    fn key_dir(&self, key: &str) -> PathBuf {
        self.root_path.join(key)
    }

    fn key_env_path(&self, key: &str) -> PathBuf {
        self.key_dir(key).join(".env")
    }
}

impl Storage for FilesystemStorage {
    fn save_env(&self, key: &str, source_path: &Path) -> Result<(), KpvError> {
        self.ensure_valid_key(key)?;
        let destination_dir = self.key_dir(key);
        fs::create_dir_all(&destination_dir)?;
        let destination_file = destination_dir.join(".env");
        fs::copy(source_path, &destination_file)?;
        Ok(())
    }

    fn link_env(&self, key: &str, dest_path: &Path) -> Result<(), KpvError> {
        self.ensure_valid_key(key)?;
        let source = self.get_key_env_path(key);
        #[cfg(unix)]
        {
            match std::os::unix::fs::symlink(&source, dest_path) {
                Ok(_) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    Err(KpvError::EnvAlreadyExists)
                }
                Err(e) => Err(KpvError::from(e)),
            }
        }
        #[cfg(windows)]
        {
            match std::os::windows::fs::symlink_file(&source, dest_path) {
                Ok(_) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    Err(KpvError::EnvAlreadyExists)
                }
                Err(e) => Err(KpvError::from(e)),
            }
        }
    }

    fn list_keys(&self) -> Result<Vec<String>, KpvError> {
        if !self.root_path.exists() {
            return Ok(Vec::new());
        }

        let mut keys = Vec::new();
        for entry in fs::read_dir(&self.root_path)? {
            let entry = entry?;
            if entry.path().is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                keys.push(name.to_string());
            }
        }

        Ok(keys)
    }

    fn get_key_env_path(&self, key: &str) -> PathBuf {
        self.key_env_path(key)
    }

    fn check_key_exists(&self, key: &str) -> bool {
        if !Self::is_key_valid(key) {
            return false;
        }
        self.key_env_path(key).exists()
    }

    fn delete_env(&self, key: &str) -> Result<(), KpvError> {
        self.ensure_valid_key(key)?;
        let key_dir = self.key_dir(key);
        if !key_dir.exists() {
            return Ok(());
        }
        fs::remove_dir_all(&key_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::ffi::OsString;
    use std::io::Write;
    use tempfile::TempDir;

    struct TestContext {
        root: TempDir,
        original_home: Option<OsString>,
        work_dir: PathBuf,
    }

    impl TestContext {
        fn new() -> Self {
            let root = TempDir::new().expect("failed to create temp dir");
            let original_home = std::env::var_os("HOME");
            unsafe {
                std::env::set_var("HOME", root.path());
            }

            let work_dir = root.path().join("work");
            fs::create_dir_all(&work_dir).expect("failed to create work dir");

            Self { root, original_home, work_dir }
        }

        fn storage(&self) -> FilesystemStorage {
            FilesystemStorage::new_default().expect("storage initialization should succeed")
        }

        fn work_dir(&self) -> &Path {
            &self.work_dir
        }

        fn storage_root(&self) -> PathBuf {
            self.root.path().join(".config").join("kpv")
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            match &self.original_home {
                Some(value) => unsafe {
                    std::env::set_var("HOME", value);
                },
                None => unsafe {
                    std::env::remove_var("HOME");
                },
            }
        }
    }

    #[test]
    #[serial]
    fn save_env_copies_file() {
        let ctx = TestContext::new();
        let storage = ctx.storage();
        let source = ctx.work_dir().join(".env");
        let mut file = fs::File::create(&source).expect("failed to create .env source");
        writeln!(file, "TEST_VAR=test").expect("failed to write .env source");

        storage.save_env("test-key", &source).expect("save_env should succeed");

        let saved_path = ctx.storage_root().join("test-key").join(".env");
        let content = fs::read_to_string(saved_path).expect("failed to read saved env");
        assert!(content.contains("TEST_VAR=test"));
    }

    #[test]
    #[serial]
    fn save_env_missing_source_errors() {
        let ctx = TestContext::new();
        let storage = ctx.storage();
        let missing = ctx.work_dir().join("missing.env");
        let result = storage.save_env("test-key", &missing);
        assert!(matches!(result, Err(KpvError::Io(_))));
    }

    #[test]
    #[serial]
    fn list_keys_empty_when_root_missing() {
        let ctx = TestContext::new();
        // Remove storage root to simulate fresh install
        fs::remove_dir_all(ctx.storage_root()).ok();

        let storage = ctx.storage();
        let keys = storage.list_keys().expect("list_keys should succeed with empty root");
        assert!(keys.is_empty());
    }

    #[test]
    #[serial]
    fn list_keys_collects_directories() {
        let ctx = TestContext::new();
        let storage_root = ctx.storage_root();
        fs::create_dir_all(storage_root.join("key2")).unwrap();
        fs::create_dir_all(storage_root.join("key1")).unwrap();
        fs::File::create(storage_root.join("file.txt")).unwrap();

        let storage = ctx.storage();
        let mut keys = storage.list_keys().expect("list_keys should succeed");
        keys.sort();
        assert_eq!(keys, vec!["key1", "key2"]);
    }
}
