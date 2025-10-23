mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;

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
