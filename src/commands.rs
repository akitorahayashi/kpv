use std::fs;
use std::io;
use std::path::PathBuf;

/// Get the storage directory path (~/.config/kpv/)
fn get_storage_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".config").join("kpv")
}

/// Get the path to a key's storage directory
fn get_key_dir(key: &str) -> PathBuf {
    get_storage_dir().join(key)
}

/// Get the path to a key's .env file
fn get_key_env_path(key: &str) -> PathBuf {
    get_key_dir(key).join(".env")
}

/// Save command: Copy ./.env to ~/.config/kpv/<key>/.env
pub fn save(key: &str) -> io::Result<()> {
    let source = PathBuf::from(".env");

    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No .env file found in the current directory",
        ));
    }

    let dest_dir = get_key_dir(key);
    let dest_file = get_key_env_path(key);

    // Create destination directory if it doesn't exist
    fs::create_dir_all(&dest_dir)?;

    // Copy the file
    fs::copy(&source, &dest_file)?;

    println!("✅ Saved: ./.env -> '{}'", key);
    Ok(())
}

/// Link command: Create symlink from ~/.config/kpv/<key>/.env to ./.env
pub fn link(key: &str) -> io::Result<()> {
    let source = get_key_env_path(key);

    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No .env file found for key '{}'", key),
        ));
    }

    let dest = PathBuf::from(".env");

    if dest.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            ".env file already exists in the current directory",
        ));
    }

    // Create symbolic link
    #[cfg(unix)]
    std::os::unix::fs::symlink(&source, &dest)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&source, &dest)?;

    println!("🔗 Linked: '{}' -> ./.env", key);
    Ok(())
}

/// List command: List all keys in ~/.config/kpv/
pub fn list() -> io::Result<()> {
    let storage_dir = get_storage_dir();

    if !storage_dir.exists() {
        println!("📦 Saved keys:");
        println!("(none)");
        return Ok(());
    }

    let entries = fs::read_dir(storage_dir)?;
    let mut keys: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    e.file_name().to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
        })
        .collect();

    println!("📦 Saved keys:");
    if keys.is_empty() {
        println!("(none)");
    } else {
        keys.sort();
        for key in keys {
            println!("- {}", key);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Write;

    fn setup_test_env() -> (PathBuf, PathBuf) {
        let test_dir = env::temp_dir().join(format!("kpv_test_{}", std::process::id()));
        let storage_dir = test_dir.join(".config").join("kpv");

        fs::create_dir_all(&test_dir).unwrap();
        unsafe {
            env::set_var("HOME", &test_dir);
        }

        (test_dir, storage_dir)
    }

    fn cleanup_test_env(test_dir: &PathBuf) {
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_save_creates_env_file() {
        let (test_dir, storage_dir) = setup_test_env();
        let work_dir = test_dir.join("work");
        fs::create_dir_all(&work_dir).unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&work_dir).unwrap();

        // Create a test .env file
        let env_path = work_dir.join(".env");
        let mut env_file = fs::File::create(&env_path).unwrap();
        writeln!(env_file, "TEST_VAR=test_value").unwrap();
        drop(env_file); // Ensure file is flushed

        // Save it
        save("test-key").unwrap();

        // Verify it was saved
        let saved_path = storage_dir.join("test-key").join(".env");
        assert!(saved_path.exists());

        let content = fs::read_to_string(&saved_path).unwrap();
        assert!(content.contains("TEST_VAR=test_value"));

        env::set_current_dir(original_dir).unwrap();
        cleanup_test_env(&test_dir);
    }

    #[test]
    fn test_save_without_env_file() {
        let (test_dir, _) = setup_test_env();
        let work_dir = test_dir.join("work2");
        fs::create_dir_all(&work_dir).unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&work_dir).unwrap();

        let result = save("test-key");
        assert!(result.is_err());

        env::set_current_dir(original_dir).unwrap();
        cleanup_test_env(&test_dir);
    }

    #[test]
    fn test_list_empty() {
        let (test_dir, _) = setup_test_env();

        // Should not panic with no storage directory
        let result = list();
        assert!(result.is_ok());

        cleanup_test_env(&test_dir);
    }

    #[test]
    fn test_list_with_keys() {
        let (test_dir, storage_dir) = setup_test_env();

        // Create some test keys
        fs::create_dir_all(storage_dir.join("key1")).unwrap();
        fs::create_dir_all(storage_dir.join("key2")).unwrap();

        let result = list();
        assert!(result.is_ok());

        cleanup_test_env(&test_dir);
    }
}
