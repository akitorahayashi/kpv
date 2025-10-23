mod common;

use common::TestContext;
use kpv::{link, save};
use serial_test::serial;
use std::io;

#[test]
#[serial]
fn save_without_env_file_reports_not_found() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        let err = save("unit-missing").expect_err("save should fail when .env is absent");
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    });
}

#[test]
#[serial]
fn link_without_saved_key_reports_not_found() {
    let ctx = TestContext::new();

    ctx.with_dir(ctx.work_dir(), || {
        let err = link("unit-missing").expect_err("link should fail when key is missing");
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    });
}
