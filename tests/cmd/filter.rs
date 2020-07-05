use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_filter_column_larger_than_a() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "name,id\noem,42\nfoo,12\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("id>13");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff("foo,12"))
        .stdout(predicate::str::contains("name,id\noem,42"));
    Ok(())
}

#[test]
fn test_filter_column_larger_than_b() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "name,id\noem,9\nfoo,1\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("id>2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("name,id\noem,9"));
    Ok(())
}

#[test]
fn test_filter_different_column() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "age,name\n140,foo\n42,goo\n21,moo\n")?;
    let mut cmd = Command::cargo_bin("atm")?;
    cmd.arg(file.path()).arg("filter").arg("age>22");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("age,name\n140,foo\n42,goo"));
    Ok(())
}
