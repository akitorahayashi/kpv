use crate::common::TestContext;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn link_with_explicit_key_creates_symlink() {
    let ctx = TestContext::new();
    // 1. Save to the current directory
    ctx.write_env_file("DEFAULT_KEY=true\n");
    ctx.cli().arg("save").assert().success();

    // 2. Link in a different workspace using explicit key
    let link_workspace = ctx.create_workspace("link-explicit-workspace");
    ctx.cli_in(&link_workspace)
        .arg("link")
        .arg("work")
        .assert()
        .success()
        .stdout(predicate::str::contains("Linked: 'work' -> ./.env"));

    let link_path = link_workspace.join(".env");
    assert!(link_path.exists(), "Expected .env link to be created");
    #[cfg(unix)]
    {
        assert!(link_path.is_symlink(), ".env should be a symlink");
        let target = fs::read_link(&link_path).expect("Failed to read symlink target");
        assert_eq!(target, ctx.saved_env_path("work"), "Symlink target should point to saved .env",);
    }
}

#[test]
#[serial]
fn link_uses_saved_env_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("SERVICE_KEY=xyz\n");

    ctx.with_dir(ctx.work_dir(), || {
        kpv::save(Some("sdk-link")).expect("library save should succeed");
    });

    let link_workspace = ctx.create_workspace("sdk-link-workspace");
    ctx.with_dir(&link_workspace, || {
        kpv::link(Some("sdk-link")).expect("library link should succeed");
    });

    let link_path = link_workspace.join(".env");
    assert!(link_path.exists(), "Link target should exist");
    #[cfg(unix)]
    {
        assert!(link_path.is_symlink(), ".env should be a symlink");
        let target = fs::read_link(&link_path).expect("Failed to inspect symlink");
        assert_eq!(target, ctx.saved_env_path("sdk-link"), "Unexpected symlink target",);
    }
}

#[test]
#[serial]
fn link_without_saved_key_reports_not_found() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        let err =
            kpv::link(Some("unit-missing")).expect_err("link should fail when key is missing");
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[test]
#[serial]
fn user_can_save_link_and_list_end_to_end() {
    let ctx = TestContext::new();
    ctx.write_env_file("TOKEN=super-secret\n");

    ctx.cli()
        .arg("save")
        .arg("e2e-project")
        .assert()
        .success()
        .stdout(predicate::str::contains("Saved: ./.env -> 'e2e-project'"));

    let link_workspace = ctx.create_workspace("e2e-link-workspace");
    ctx.cli_in(&link_workspace)
        .arg("link")
        .arg("e2e-project")
        .assert()
        .success()
        .stdout(predicate::str::contains("Linked: 'e2e-project' -> ./.env"));

    let link_path = link_workspace.join(".env");
    assert!(link_path.exists(), "Expected .env link to be created");
    #[cfg(unix)]
    {
        assert!(link_path.is_symlink(), ".env should be a symlink");
        let target = fs::read_link(&link_path).expect("Failed to read symlink target");
        assert_eq!(
            target,
            ctx.saved_env_path("e2e-project"),
            "Symlink target should point to saved .env",
        );
    }

    ctx.cli().arg("list").assert().success().stdout(predicate::str::contains("- e2e-project"));
}
