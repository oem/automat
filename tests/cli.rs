use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn filter_without_conditions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("filter");
    cmd.assert().failure();

    Ok(())
}
