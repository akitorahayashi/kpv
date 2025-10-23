use std::path::PathBuf;

use crate::core::{self, Execute};
use crate::error::KpvError;
use crate::storage::FilesystemStorage;

/// Save command: Copy ./.env to ~/.config/kpv/<key>/.env
pub fn save(key: &str) -> Result<(), KpvError> {
    let storage = FilesystemStorage::new_default()?;
    let source = PathBuf::from(".env");
    let command = core::save::SaveCommand { key, source_path: &source };

    command.execute(&storage)?;
    println!("✅ Saved: ./.env -> '{}'", key);
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
