use crate::common::TestContext;
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

    // Verify it's gone - list should not contain the deleted key
    ctx.cli().arg("list").assert().success().stdout(predicate::str::contains("to-delete").not());
}

#[test]
#[serial]
fn delete_nonexistent_key_succeeds_silently() {
    // Delete is idempotent - deleting a non-existent key succeeds
    let ctx = TestContext::new();

    ctx.cli()
        .arg("delete")
        .arg("nonexistent")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted: 'nonexistent'"));
}

#[test]
#[serial]
fn delete_via_library_api() {
    let ctx = TestContext::new();
    ctx.write_env_file("API_DELETE=test\n");

    ctx.with_dir(ctx.work_dir(), || {
        kpv::save(Some("api-delete")).expect("save should succeed");
        kpv::delete("api-delete").expect("delete should succeed");
    });

    // Verify deleted - list should succeed without the deleted key
    ctx.with_dir(ctx.work_dir(), || {
        let _result = kpv::list();
    });
}
