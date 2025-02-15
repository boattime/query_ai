#[cfg(test)]
mod tests {
    use repositories::load_config;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn test_load_config_from_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config_content = r#"
        [repository]
        repo_url = "https://github.com/example/test-repo.git"
        local_path = "repos/test-repo"
        "#;

        fs::write(&config_path, config_content).unwrap();
        let config = load_config(Some(config_path.to_str().unwrap())).unwrap();

        assert_eq!(config.repo_url, "https://github.com/example/test-repo.git");
        assert_eq!(config.local_path, "repos/test-repo");
    }

    #[test]
    #[serial]
    fn test_load_config_from_env() {
        env::set_var("REPO_URL", "https://github.com/example/env-repo.git");
        env::set_var("LOCAL_PATH", "repos/env-repo");

        let config = load_config(None).unwrap();

        assert_eq!(config.repo_url, "https://github.com/example/env-repo.git");
        assert_eq!(config.local_path, "repos/env-repo");

        env::remove_var("REPO_URL");
        env::remove_var("LOCAL_PATH");
    }

    #[test]
    #[serial]
    fn test_load_config_file_missing() {
        let result = load_config(Some("nonexistent.toml"));
        assert!(
            result.is_err(),
            "Expected error when config file is missing"
        );
    }

    #[test]
    #[serial]
    fn test_load_config_file_malformed() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("bad_config.toml");

        let bad_config_content = r#"
        [repository]
        repo_url = "https://github.com/example/malformed
        local_path = "repos/malformed"
        "#;

        fs::write(&config_path, bad_config_content).unwrap();
        let result = load_config(Some(config_path.to_str().unwrap()));

        assert!(
            result.is_err(),
            "Expected error when parsing a malformed TOML file"
        );
    }

    #[test]
    #[serial]
    fn test_load_config_env_missing() {
        env::remove_var("REPO_URL");
        env::remove_var("LOCAL_PATH");

        let result = load_config(None);
        assert!(
            result.is_err(),
            "Expected error when environment variables are missing"
        );
    }
}
