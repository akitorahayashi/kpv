mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn save_and_list() {
    let ctx = TestContext::new();
    ctx.write_env_file("API_KEY=secret123\n");

    ctx.cli()
        .arg("save")
        .arg("test-project")
        .assert()
        .success()
        .stdout(predicate::str::contains("Saved: ./.env -> 'test-project'"));

    ctx.assert_saved_env_contains("test-project", "API_KEY=secret123");

    ctx.cli().arg("list").assert().success().stdout(predicate::str::contains("test-project"));
}

#[test]
#[serial]
fn link_creates_symlink() {
    let ctx = TestContext::new();
    ctx.write_env_file("DATABASE_URL=postgres://localhost\n");

    ctx.cli().arg("save").arg("db-project").assert().success();

    let link_workspace = ctx.create_workspace("link-workspace");

    ctx.cli_in(&link_workspace)
        .arg("link")
        .arg("db-project")
        .assert()
        .success()
        .stdout(predicate::str::contains("Linked: 'db-project' -> ./.env"));

    let link_path = link_workspace.join(".env");
    assert!(link_path.exists(), "Expected symlink to be created");
    #[cfg(unix)]
    {
        assert!(link_path.is_symlink(), "Expected .env to be a symlink");
        let target = fs::read_link(&link_path).expect("Failed to read symlink target");
        assert_eq!(target, ctx.saved_env_path("db-project"), "Symlink target mismatch");
    }
}

#[test]
#[serial]
fn save_without_env_file_fails() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("save")
        .arg("test-project")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .env file found"));
}

#[test]
#[serial]
fn link_existing_env_error() {
    let ctx = TestContext::new();
    ctx.write_env_file("TEST=value\n");

    ctx.cli().arg("save").arg("existing-project").assert().success();

    let link_workspace = ctx.create_workspace("occupied-workspace");
    ctx.touch_env_in(&link_workspace);

    ctx.cli_in(&link_workspace)
        .arg("link")
        .arg("existing-project")
        .assert()
        .failure()
        .stderr(predicate::str::contains(".env file already exists"));
}

#[test]
#[serial]
fn list_empty_succeeds() {
    let ctx = TestContext::new();

    ctx.cli().arg("list").assert().success().stdout(predicate::str::contains("Saved keys:"));
}

#[test]
#[serial]
fn delete_removes_saved_key() {
    let ctx = TestContext::new();
    ctx.write_env_file("TO_DELETE=value\n");

    ctx.cli().arg("save").arg("delete-me").assert().success();

    assert!(ctx.saved_env_path("delete-me").exists(), "Key should exist before delete");

    ctx.cli()
        .arg("delete")
        .arg("delete-me")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted: 'delete-me'"));

    assert!(!ctx.saved_env_path("delete-me").exists(), "Key should not exist after delete");
}

#[test]
#[serial]
fn delete_with_rm_alias() {
    let ctx = TestContext::new();
    ctx.write_env_file("ALIAS_TEST=data\n");

    ctx.cli().arg("save").arg("alias-key").assert().success();

    ctx.cli()
        .arg("rm")
        .arg("alias-key")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted: 'alias-key'"));

    assert!(!ctx.saved_env_path("alias-key").exists(), "Key should be deleted using rm alias");
}

#[test]
#[serial]
fn delete_nonexistent_key_succeeds() {
    let ctx = TestContext::new();

    ctx.cli().arg("delete").arg("does-not-exist").assert().success();
}
