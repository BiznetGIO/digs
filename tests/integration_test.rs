use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("digs ● dig many at once"));
}

#[test]
fn config_not_found() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net").arg("-f").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn config_invalid() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/invalid.toml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid config file."));
}

#[test]
fn rtype_invalid() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("FOO")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"'FOO' isn't a valid value for '<rtype>'"#,
    ));
}

#[test]
fn rtype_too_many() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("A")
        .arg("MX")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Found argument 'MX' which wasn't expected",
    ));
}

#[test]
fn address_invalid() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/invalid-address.toml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid addres"));
}

#[test]
fn query() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("A")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
}

// should fallback to A
#[test]
fn query_without_rtype() {
    let mut cmd = Command::cargo_bin("digs").unwrap();
    cmd.arg("example.net")
        .arg("-f")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
}
