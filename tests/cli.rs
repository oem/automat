use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_help_includes_filter() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("filter"));
    Ok(())
}

#[test]
fn test_filter_without_conditions() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("filter");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("condition"));
    Ok(())
}

#[test]
fn test_filter_with_conditions() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "name,id\noem,42\nfoo,12\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("id<12");
    cmd.assert().success();
    Ok(())
}

#[test]
fn test_filter_from_stdin() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg("filter")
        .arg("id<12")
        .write_stdin("name,id\noem,11\nfoo,12\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,id\noem,11"));
    Ok(())
}
