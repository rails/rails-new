use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn requires_a_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rails-new")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<NAME>"));

    Ok(())
}
