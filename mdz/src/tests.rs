#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use predicates::prelude::*;
    use assert_cmd::Command;

    #[allow(deprecated)]
    fn cargo_bin(name: &str) -> Command {
        Command::cargo_bin(name).unwrap()
    }

    #[test]
    fn test_cli_help() {
        cargo_bin("mdz")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("Usage"));
    }

    #[test]
    fn test_cli_version() {
        cargo_bin("mdz")
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("mdz"))
            .stdout(predicate::str::contains("1.0.0"));
    }

    #[test]
    fn test_cli_pack_help() {
        cargo_bin("mdz")
            .arg("pack")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("pack"));
    }

    #[test]
    fn test_cli_unpack_help() {
        cargo_bin("mdz")
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

        cargo_bin("mdz")
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

        cargo_bin("mdz")
            .arg("unpack")
            .arg("nonexistent.mdz")
            .arg("--output").arg(output_dir.to_str().unwrap())
            .assert()
            .failure();
    }
}