mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;

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
fn save_without_key_defaults_to_directory_name() {
    let ctx = TestContext::new();
    ctx.write_env_file("FOO=bar\n");

    ctx.cli()
        .arg("save")
        .assert()
        .success()
        .stdout(predicate::str::contains("Saved: ./.env -> 'work'"));

    ctx.assert_saved_env_contains("work", "FOO=bar");

    ctx.cli().arg("list").assert().success().stdout(predicate::str::contains("work"));
}

#[test]
#[serial]
fn save_without_env_file_reports_not_found() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        let err = kpv::save(Some("unit-missing")).expect_err("save should fail when .env is absent");
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[test]
#[serial]
fn save_persists_env_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("FOO=bar\n");

    ctx.with_dir(ctx.work_dir(), || {
        kpv::save(Some("sdk-save")).expect("library save should succeed");
    });

    ctx.assert_saved_env_contains("sdk-save", "FOO=bar");
}