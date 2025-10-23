mod common;

use common::TestContext;
use kpv::{delete, link, list, save};
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn save_persists_env_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("FOO=bar\n");

    ctx.with_dir(ctx.work_dir(), || {
        save("sdk-save").expect("library save should succeed");
    });

    ctx.assert_saved_env_contains("sdk-save", "FOO=bar");
}

#[test]
#[serial]
fn link_uses_saved_env_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("SERVICE_KEY=xyz\n");

    ctx.with_dir(ctx.work_dir(), || {
        save("sdk-link").expect("library save should succeed");
    });

    let link_workspace = ctx.create_workspace("sdk-link-workspace");
    ctx.with_dir(&link_workspace, || {
        link("sdk-link").expect("library link should succeed");
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
fn list_returns_ok_via_library_api() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        list().expect("listing should not fail even with empty storage");
    });

    ctx.write_env_file("KVP=value\n");
    ctx.with_dir(ctx.work_dir(), || {
        save("sdk-list").expect("save should succeed");
        list().expect("listing should succeed with stored keys");
    });
}

#[test]
#[serial]
fn delete_removes_saved_key_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("TEMP=data\n");

    ctx.with_dir(ctx.work_dir(), || {
        save("sdk-delete").expect("save should succeed");
    });

    assert!(ctx.saved_env_path("sdk-delete").exists(), "Saved env should exist before delete");

    ctx.with_dir(ctx.work_dir(), || {
        delete("sdk-delete").expect("library delete should succeed");
    });

    assert!(!ctx.saved_env_path("sdk-delete").exists(), "Saved env should not exist after delete");
}

#[test]
#[serial]
fn delete_nonexistent_key_succeeds_via_library_api() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        delete("nonexistent-key").expect("deleting nonexistent key should not fail");
    });
}
