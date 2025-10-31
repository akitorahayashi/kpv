use std::env;
use std::path::PathBuf;

use crate::core::{self, Execute};
use crate::error::KpvError;
use crate::storage::FilesystemStorage;

fn derive_dir_name() -> Result<String, KpvError> {
    let current_path = env::current_dir()?;
    let dir_os = current_path.file_name().ok_or_else(|| {
        KpvError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine current directory name",
        ))
    })?;

    let dir_str = dir_os.to_str().ok_or_else(|| {
        KpvError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Current directory name is not valid UTF-8",
        ))
    })?;

    Ok(dir_str.to_string())
}

/// Save command: Copy ./.env to ~/.config/kpv/<key>/.env
pub fn save(key_opt: Option<&str>) -> Result<(), KpvError> {
    let storage = FilesystemStorage::new_default()?;
    let source = PathBuf::from(".env");

    let key_owned;
    let key_to_save = if let Some(key) = key_opt {
        key
    } else {
        key_owned = derive_dir_name()?;
        &key_owned
    };

    let command = core::save::SaveCommand { key: key_to_save, source_path: &source };

    command.execute(&storage)?;
    println!("✅ Saved: ./.env -> '{}'", key_to_save);
    Ok(())
}

/// Link command: Create symlink from ~/.config/kpv/<key>/.env to ./.env
pub fn link(key: &str) -> Result<(), KpvError> {
    let storage = FilesystemStorage::new_default()?;
    let dest = PathBuf::from(".env");
    let command = core::link::LinkCommand { key, dest_path: &dest };

    command.execute(&storage)?;
    println!("🔗 Linked: '{}' -> ./.env", key);
    Ok(())
}

/// List command: List all keys in ~/.config/kpv/
pub fn list() -> Result<(), KpvError> {
    let storage = FilesystemStorage::new_default()?;
    let command = core::list::ListCommand;
    let keys = command.execute(&storage)?;

    println!("📦 Saved keys:");
    if keys.is_empty() {
        println!("(none)");
    } else {
        for key in keys {
            println!("- {}", key);
        }
    }

    Ok(())
}

/// Delete command: Remove a saved key from ~/.config/kpv/<key>
pub fn delete(key: &str) -> Result<(), KpvError> {
    let storage = FilesystemStorage::new_default()?;
    let command = core::delete::DeleteCommand { key };

    command.execute(&storage)?;
    println!("🗑️  Deleted: '{}'", key);
    Ok(())
}
