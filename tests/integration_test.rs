use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{error::Error, process::Command};

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("dig many at once"));
    Ok(())
}

#[test]
fn default_config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net").arg("-f").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net").arg("-f").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn config_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/invalid.toml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid config"));
    Ok(())
}

#[test]
fn rtype_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("FOO")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"'FOO' isn't a valid value for '[RTYPE]'"#,
    ));
    Ok(())
}

#[test]
fn rtype_too_many() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("A")
        .arg("MX")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Found argument 'MX' which wasn't expected",
    ));
    Ok(())
}

#[test]
fn address_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/invalid-address.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invalid IP address"));
    Ok(())
}

#[test]
fn domain_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example")
        .arg("-f")
        .arg("tests/fixture/invalid-address.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"Error: Invalid domain "example""#,
    ));
    Ok(())
}

#[test]
fn query() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("A")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
    Ok(())
}

// should fallback to A
#[test]
fn query_without_rtype() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
    Ok(())
}
