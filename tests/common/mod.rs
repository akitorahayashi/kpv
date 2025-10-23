//! Shared testing utilities mirroring the reference project's fixture culture.

use assert_cmd::Command;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Testing harness providing an isolated HOME/workspace pair for CLI and SDK exercises.
#[allow(dead_code)]
pub struct TestContext {
    root: TempDir,
    work_dir: PathBuf,
    original_home: Option<OsString>,
}

#[allow(dead_code)]
impl TestContext {
    /// Create a new isolated environment and point `HOME` to it so the CLI uses local storage.
    pub fn new() -> Self {
        let root = TempDir::new().expect("Failed to create temp directory for tests");
        let work_dir = root.path().join("work");
        fs::create_dir_all(&work_dir).expect("Failed to create test work directory");

        let original_home = env::var_os("HOME");
        unsafe {
            env::set_var("HOME", root.path());
        }

        Self { root, work_dir, original_home }
    }

    /// Absolute path to the emulated `$HOME` directory.
    pub fn home(&self) -> &Path {
        self.root.path()
    }

    /// Path to the workspace directory used for CLI invocations.
    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    /// Convenience helper to create additional sibling workspaces (e.g., for linking scenarios).
    pub fn create_workspace(&self, name: &str) -> PathBuf {
        let path = self.home().join(name);
        fs::create_dir_all(&path).expect("Failed to create additional workspace");
        path
    }

    /// Populate the default workspace with a `.env` file containing the provided contents.
    pub fn write_env_file(&self, contents: &str) {
        let env_path = self.work_dir().join(".env");
        fs::write(&env_path, contents).expect("Failed to write .env file for test");
    }

    /// Create an empty `.env` file in the given directory.
    pub fn touch_env_in<P: AsRef<Path>>(&self, dir: P) {
        let path = dir.as_ref().join(".env");
        fs::File::create(path).expect("Failed to touch .env file");
    }

    /// Build a command for invoking the compiled `kpv` binary within the default workspace.
    pub fn cli(&self) -> Command {
        self.cli_in(self.work_dir())
    }

    /// Build a command for invoking the compiled `kpv` binary within a custom directory.
    pub fn cli_in<P: AsRef<Path>>(&self, dir: P) -> Command {
        let mut cmd = Command::cargo_bin("kpv").expect("Failed to locate kpv binary");
        cmd.current_dir(dir.as_ref()).env("HOME", self.home());
        cmd
    }

    /// Return the path where the CLI stores a saved `.env` file for the provided key.
    pub fn saved_env_path(&self, key: &str) -> PathBuf {
        self.home().join(".config").join("kpv").join(key).join(".env")
    }

    /// Assert that a saved `.env` contains the provided value snippet.
    pub fn assert_saved_env_contains(&self, key: &str, expected_snippet: &str) {
        let env_path = self.saved_env_path(key);
        assert!(env_path.exists(), "Expected saved .env at {}", env_path.display());
        let content = fs::read_to_string(&env_path).expect("Failed to read saved .env");
        assert!(
            content.contains(expected_snippet),
            "Saved .env for key `{key}` did not contain `{expected}`; content: {content}",
            expected = expected_snippet
        );
    }

    /// Execute a closure after temporarily switching into the provided directory.
    pub fn with_dir<F, R, P>(&self, dir: P, action: F) -> R
    where
        F: FnOnce() -> R,
        P: AsRef<Path>,
    {
        let original = env::current_dir().expect("Failed to capture current dir");
        env::set_current_dir(dir.as_ref()).expect("Failed to switch current dir");
        let result = action();
        env::set_current_dir(original).expect("Failed to restore current dir");
        result
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        match &self.original_home {
            Some(value) => unsafe {
                env::set_var("HOME", value);
            },
            None => unsafe {
                env::remove_var("HOME");
            },
        }
    }
}
