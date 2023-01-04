use std::{error::Error, process::Command};

use assert_cmd::{crate_name, prelude::*};
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("dig many at once"));
    Ok(())
}

#[test]
fn default_config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net")
        .arg("-c")
        .arg("file/doesnt/exist/digs.toml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Configuration file is not found"));
    Ok(())
}

#[test]
fn config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net").arg("-c").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Configuration file is not found"));
    Ok(())
}

#[test]
fn config_invalid() -> Result<(), Box<dyn Error>> {
    let content = r#"
[[servers]]
# missing name
ip = "1.1.1.1"

[[servers]]
ip = "8.8.8.8"
name = "Google"

[[servers]]
ip = "9.9.9.9"
name = "Quad9"
"#;

    let mut cmd = Command::cargo_bin(crate_name!())?;

    let temp_dir = assert_fs::TempDir::new()?;
    let config = temp_dir.child("invalid.toml");
    config.write_str(content)?;

    cmd.arg("example.net").arg("-c").arg(config.to_path_buf());
    cmd.assert().failure().stderr(predicate::str::contains(
        "Invalid configuration: missing field `name`",
    ));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn rtype_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net")
        .arg("FOO")
        .arg("-c")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"'FOO' isn't a valid value for '[RTYPE]'"#,
    ));
    Ok(())
}

#[test]
fn rtype_too_many() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net")
        .arg("A")
        .arg("MX")
        .arg("-c")
        .arg("tests/fixture/digs.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Found argument 'MX' which wasn't expected",
    ));
    Ok(())
}

#[test]
fn address_invalid() -> Result<(), Box<dyn Error>> {
    let content = r#"
[[servers]]
# invalid address
ip = "8.8.8"
name = "Google"
"#;

    let mut cmd = Command::cargo_bin(crate_name!())?;

    let temp_dir = assert_fs::TempDir::new()?;
    let config = temp_dir.child("invalid.toml");
    config.write_str(content)?;

    cmd.arg("example.net").arg("-c").arg(config.to_path_buf());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invalid IP Adrress `8.8.8"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn domain_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r#"Invalid domain "example""#));
    Ok(())
}

#[test]
fn query() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net")
        .arg("A")
        .arg("-c")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
    Ok(())
}

// should fallback to A
#[test]
fn query_without_rtype() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net")
        .arg("-c")
        .arg("tests/fixture/digs.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("184.216.34"));
    Ok(())
}
