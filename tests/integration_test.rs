use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove 'deps'
    path.push("kpv");
    path
}

fn setup_test_environment() -> (PathBuf, PathBuf) {
    let test_dir = env::temp_dir().join(format!("kpv_integration_test_{}", std::process::id()));
    let work_dir = test_dir.join("work");
    fs::create_dir_all(&work_dir).unwrap();
    (test_dir, work_dir)
}

fn cleanup_test_environment(test_dir: &PathBuf) {
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_save_and_list() {
    let (test_dir, work_dir) = setup_test_environment();
    env::set_current_dir(&work_dir).unwrap();

    // Create a .env file
    let mut env_file = fs::File::create(work_dir.join(".env")).unwrap();
    writeln!(env_file, "API_KEY=secret123").unwrap();

    // Run save command
    let output = Command::new(get_binary_path())
        .arg("save")
        .arg("test-project")
        .env("HOME", &test_dir)
        .current_dir(&work_dir)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Saved: ./.env -> 'test-project'"));

    // Verify file exists in storage
    let saved_path = test_dir.join(".config/kpv/test-project/.env");
    assert!(saved_path.exists());

    // Run list command
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", &test_dir)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test-project"));

    cleanup_test_environment(&test_dir);
}

#[test]
fn test_link() {
    let (test_dir, work_dir) = setup_test_environment();

    // Setup: save an env file first
    let save_dir = test_dir.join("save_dir");
    fs::create_dir_all(&save_dir).unwrap();

    let mut env_file = fs::File::create(save_dir.join(".env")).unwrap();
    writeln!(env_file, "DATABASE_URL=postgres://localhost").unwrap();

    Command::new(get_binary_path())
        .arg("save")
        .arg("db-project")
        .env("HOME", &test_dir)
        .current_dir(&save_dir)
        .output()
        .unwrap();

    // Now try to link it in a different directory
    env::set_current_dir(&work_dir).unwrap();

    let output = Command::new(get_binary_path())
        .arg("link")
        .arg("db-project")
        .env("HOME", &test_dir)
        .current_dir(&work_dir)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Linked: 'db-project' -> ./.env"));

    // Verify symlink was created
    let link_path = work_dir.join(".env");
    assert!(link_path.exists());
    assert!(link_path.is_symlink());

    cleanup_test_environment(&test_dir);
}

#[test]
fn test_save_without_env_file() {
    let (test_dir, work_dir) = setup_test_environment();
    env::set_current_dir(&work_dir).unwrap();

    let output = Command::new(get_binary_path())
        .arg("save")
        .arg("test-project")
        .env("HOME", &test_dir)
        .current_dir(&work_dir)
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No .env file found"));

    cleanup_test_environment(&test_dir);
}

#[test]
fn test_link_existing_env_error() {
    let (test_dir, work_dir) = setup_test_environment();

    // Setup: save an env file first
    let save_dir = test_dir.join("save_dir");
    fs::create_dir_all(&save_dir).unwrap();

    let mut env_file = fs::File::create(save_dir.join(".env")).unwrap();
    writeln!(env_file, "TEST=value").unwrap();

    Command::new(get_binary_path())
        .arg("save")
        .arg("existing-project")
        .env("HOME", &test_dir)
        .current_dir(&save_dir)
        .output()
        .unwrap();

    // Create an existing .env in work_dir
    env::set_current_dir(&work_dir).unwrap();
    fs::File::create(work_dir.join(".env")).unwrap();

    // Try to link - should fail
    let output = Command::new(get_binary_path())
        .arg("link")
        .arg("existing-project")
        .env("HOME", &test_dir)
        .current_dir(&work_dir)
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(".env file already exists"));

    cleanup_test_environment(&test_dir);
}

#[test]
fn test_list_empty() {
    let (test_dir, _) = setup_test_environment();

    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", &test_dir)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Saved keys:"));

    cleanup_test_environment(&test_dir);
}
