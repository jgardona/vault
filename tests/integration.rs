use std::{fs, path::Path};

use anyhow::{Ok, Result};
use assert_cmd::Command;
use assert_fs::prelude::{FileWriteBin, PathAssert, PathChild};
use predicates::prelude::predicate;

const FILE_PATH: &str = "tests/store.json";

#[test]
fn it_works() {}

#[test]
fn test_create_store() -> Result<()> {
    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("create").arg(FILE_PATH).assert();
    let path = Path::new(FILE_PATH);
    assert!(path.exists());
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_insert() -> Result<()> {
    const CONTENT: &[u8] = br#"{"data":{}}"#;

    let tmp = assert_fs::TempDir::new()?;
    let file = tmp.child("test_insert.json");
    file.write_binary(CONTENT)?;

    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("insert")
        .arg(file.path())
        .arg("user1")
        .arg("123456")
        .arg("key one")
        .assert()
        .success();

    file.assert(predicate::str::contains("123456"));
    tmp.close()?;

    Ok(())
}

#[test]
fn test_size() -> Result<()> {
    const CONTENT: &[u8] = br#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;
    let tmp = assert_fs::TempDir::new()?;
    let file = tmp.child("test_size.json");
    file.write_binary(CONTENT)?;

    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("read")
        .arg(file.path())
        .arg("-s")
        .assert()
        .stdout(predicate::str::contains("1"))
        .success();

    tmp.close()?;
    Ok(())
}

#[test]
fn test_list() -> Result<()> {
    const CONTENT: &[u8] = br#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;
    let tmp = assert_fs::TempDir::new()?;
    let file = tmp.child("test_size.json");
    file.write_binary(CONTENT)?;

    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("read")
        .arg(file.path())
        .arg("-l")
        .assert()
        .stdout(predicate::str::contains("test@gmail.com"))
        .success();

    tmp.close()?;
    Ok(())
}

#[test]
fn test_lock_unlock() -> Result<()> {
    const CONTENT: &[u8] = br#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;
    let tmp = assert_fs::TempDir::new()?;
    let file = tmp.child("test_size.json");
    file.write_binary(CONTENT)?;

    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("lock")
        .arg(file.path())
        .arg(tmp.path().join("output.json"))
        .assert()
        .success();

    drop(cmd);

    let mut cmd2 = Command::cargo_bin("syk")?;
    cmd2.arg("unlock")
        .arg(tmp.path().join("output.json"))
        .arg(tmp.path().join("output2.json"))
        .assert()
        .success();

    tmp.close()?;

    Ok(())
}

#[test]
fn test_remove() -> Result<()> {
    const CONTENT: &[u8] = br#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;
    let tmp = assert_fs::TempDir::new()?;
    let file = tmp.child("test_size.json");
    file.write_binary(CONTENT)?;

    let mut cmd = Command::cargo_bin("syk")?;
    cmd.arg("remove")
        .arg(file.path())
        .arg("1")
        .assert()
        .success();

    file.assert(predicates::boolean::NotPredicate::new(
        predicates::str::diff("123"),
    ));

    tmp.close()?;
    Ok(())
}
