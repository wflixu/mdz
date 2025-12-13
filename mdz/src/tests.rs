#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use predicates::prelude::*;

    #[test]
    fn test_cli_help() {
        // Build the binary first to ensure it exists
        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("Usage"));
    }

    #[test]
    fn test_cli_version() {
        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("1.0.0"));
    }

    #[test]
    fn test_cli_pack_help() {
        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
            .arg("pack")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("pack"));
    }

    #[test]
    fn test_cli_unpack_help() {
        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
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

        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
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

        assert_cmd::Command::cargo_bin("mdz")
            .unwrap()
            .arg("unpack")
            .arg("nonexistent.mdz")
            .arg("--output").arg(output_dir.to_str().unwrap())
            .assert()
            .failure();
    }
}