mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn list_returns_ok_via_library_api() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        kpv::list().expect("list should succeed");
    });
}

#[test]
#[serial]
fn list_shows_saved_projects() {
    let ctx = TestContext::new();
    ctx.write_env_file("KEY=value\n");

    ctx.cli().arg("save").arg("project1").assert().success();
    ctx.cli().arg("save").arg("project2").assert().success();

    ctx.cli()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("project1"))
        .stdout(predicate::str::contains("project2"));
}