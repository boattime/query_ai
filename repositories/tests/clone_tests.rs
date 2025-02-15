#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;
    use repositories::*;

    // Mock GitRepo trait
    mock! {
        pub GitClient {}
        impl GitRepo for GitClient {
            fn clone_repo(&self, repo_url: &str, local_path: &str) -> Result<(), git2::Error>;
        }
    }

    #[test]
    fn test_clone_repository_success() {
        let mut mock_client = MockGitClient::new();

        mock_client
            .expect_clone_repo()
            .with(
                eq("https://github.com/example/test-repo.git"),
                eq("repos/test-repo"),
            )
            .returning(|_, _| Ok(()));

        let result = clone_repository(
            &mock_client,
            "https://github.com/example/test-repo.git",
            "repos/test-repo",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_clone_repository_failure() {
        let mut mock_client = MockGitClient::new();

        mock_client
            .expect_clone_repo()
            .returning(|_, _| Err(git2::Error::from_str("Cloning failed")));

        let result = clone_repository(
            &mock_client,
            "https://github.com/example/fail-repo.git",
            "repos/fail-repo",
        );
        assert!(result.is_err());
    }
}
