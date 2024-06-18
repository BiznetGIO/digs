use std::{error::Error, process::Command};

use assert_cmd::{crate_name, prelude::*};
use assert_fs::{fixture::ChildPath, prelude::*, TempDir};
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
// default config should be in current directory
fn default_config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Configuration file is not found"));
    Ok(())
}

#[test]
fn custom_config_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net").arg("-c").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Configuration file is not found"));
    Ok(())
}

#[test]
fn invalid_config() -> Result<(), Box<dyn Error>> {
    let content = r#"
[[servers]]
# missing name
address = "1.1.1.1"

[[servers]]
address = "8.8.8.8"
name = "Google"

[[servers]]
address = "9.9.9.9"
name = "Quad9"
"#;

    let (temp_dir, config) = setup_config()?;
    config.write_str(content)?;

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net").arg("-c").arg(config.to_path_buf());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("missing field `name`"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn rtype_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("digs")?;
    cmd.arg("example.net").arg("FOO");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"invalid value 'FOO' for '[RTYPE]'"#,
    ));
    Ok(())
}

#[test]
fn rtype_too_many() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net").arg("A").arg("MX");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument 'MX' found"));
    Ok(())
}

#[test]
fn address_invalid() -> Result<(), Box<dyn Error>> {
    let content = r#"
[[servers]]
# invalid address
address = "8.8.8"
name = "Google"
"#;
    let (temp_dir, config) = setup_config()?;
    config.write_str(content)?;

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example.net").arg("-c").arg(config.to_path_buf());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid socket address syntax"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn domain_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("example");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r#"example isn't a valid domain"#));
    Ok(())
}

#[test]
fn query() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_base())?;

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("github.com")
        .arg("A")
        .arg("-c")
        .arg(config.to_path_buf());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("20.205.243.166"));

    temp_dir.close()?;
    Ok(())
}

// should fallback to A
#[test]
fn query_without_rtype() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_base())?;

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("github.com").arg("-c").arg(config.to_path_buf());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("20.205.243.166"));

    temp_dir.close()?;
    Ok(())
}

fn setup_config() -> Result<(TempDir, ChildPath), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let config = temp_dir.child("config.toml");
    Ok((temp_dir, config))
}

fn config_base() -> String {
    let content = r#"
[[servers]]
address = "8.8.8.8"
name = "Google"

[[servers]]
address = "9.9.9.9:53"
name = "Quad9"
"#;
    content.to_string()
}
