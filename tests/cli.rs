use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn requires_a_name() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("rails-new")?;

  cmd
    .spawn()
    .unwrap()
    .wait_with_output()
    .expect("rails new weblog --api");

  Ok(())
}