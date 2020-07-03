use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn filter_column_larger_than_a() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "name,id\noem,42\nfoo,12\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("id>12");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff("foo,12"))
        .stdout(predicate::str::contains("name,id\noem,42"));
    Ok(())
}

#[test]
fn filter_column_larger_than_b() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "name,id\noem,9\nfoo,1\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("id>2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("name,id\noem,42"));
    Ok(())
}
