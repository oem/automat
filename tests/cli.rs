use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn filter_without_conditions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("filter");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("conditions"));
    Ok(())
}

#[test]
fn filter_with_conditions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("filter").arg("foo<12");
    cmd.assert().success();
    Ok(())
}
