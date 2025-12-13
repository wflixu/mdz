#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("--help");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("Usage"));
    }

    #[test]
    fn test_cli_version() {
        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("--version");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("1.0.0"));
    }

    #[test]
    fn test_cli_pack_help() {
        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("pack")
            .arg("--help");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("pack"));
    }

    #[test]
    fn test_cli_unpack_help() {
        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("unpack")
            .arg("--help");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("unpack"));
    }

    #[test]
    fn test_cli_pack_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let output_file = temp_path.join("test.mdz");

        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("pack")
            .arg("nonexistent.md")
            .arg("--output").arg(output_file.to_str().unwrap());

        cmd.assert().failure();
    }

    #[test]
    fn test_cli_unpack_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let output_dir = temp_path.join("output");

        let mut cmd = Command::cargo_bin("mdz").unwrap();
        cmd.arg("unpack")
            .arg("nonexistent.mdz")
            .arg("--output").arg(output_dir.to_str().unwrap());

        cmd.assert().failure();
    }
}