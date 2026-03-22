//! CLI integration tests for mdz
//!
//! Uses CARGO_BIN_EXE_mdz environment variable for robust binary lookup
//! that works with custom build-dir configurations.

use std::env;
use tempfile::TempDir;
use predicates::prelude::*;
use assert_cmd::Command;

/// Get the mdz command using the compile-time binary path
/// This approach is robust to custom build-dir configurations
fn mdz_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mdz"))
}

#[test]
fn test_cli_help() {
    mdz_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("mdz"))
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_cli_version() {
    mdz_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mdz"))
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_cli_pack_help() {
    mdz_cmd()
        .arg("pack")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("pack"));
}

#[test]
fn test_cli_unpack_help() {
    mdz_cmd()
        .arg("unpack")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("unpack"));
}

#[test]
fn test_cli_pack_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let output_file = temp_path.join("test.mdz");

    mdz_cmd()
        .arg("pack")
        .arg("nonexistent.md")
        .arg("--output").arg(output_file.to_str().unwrap())
        .assert()
        .failure();
}

#[test]
fn test_cli_unpack_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let output_dir = temp_path.join("output");

    mdz_cmd()
        .arg("unpack")
        .arg("nonexistent.mdz")
        .arg("--output").arg(output_dir.to_str().unwrap())
        .assert()
        .failure();
}
