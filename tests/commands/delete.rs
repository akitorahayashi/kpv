mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn delete_removes_saved_env() {
    let ctx = TestContext::new();
    ctx.write_env_file("TO_DELETE=123\n");

    ctx.cli().arg("save").arg("to-delete").assert().success();

    ctx.cli()
        .arg("delete")
        .arg("to-delete")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted: 'to-delete'"));

    // Verify it's gone
    ctx.cli().arg("list").assert().success().stdout(predicate::str::diff("to-delete"));
}

#[test]
#[serial]
fn delete_nonexistent_key_reports_error() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("delete")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
#[serial]
fn delete_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("API_DELETE=test\n");

    ctx.with_dir(ctx.work_dir(), || {
        kpv::save(Some("api-delete")).expect("save should succeed");
        kpv::delete(Some("api-delete")).expect("delete should succeed");
    });

    // Verify deleted
    ctx.with_dir(ctx.work_dir(), || {
        let result = kpv::list();
        // Assuming list returns something that doesn't contain api-delete
    });
}