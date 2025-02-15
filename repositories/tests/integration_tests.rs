#[cfg(test)]
mod tests {
    use git2::Repository;
    use repositories::*;
    use tempfile::tempdir;

    #[test]
    fn test_clone_local_repo() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("test_repo.git");

        // Create a bare repository
        Repository::init_bare(&repo_path).unwrap();

        let clone_path = temp_dir.path().join("cloned_repo");
        let result = clone_repository(
            &GitClient,
            repo_path.to_str().unwrap(),
            clone_path.to_str().unwrap(),
        );

        assert!(result.is_ok());
        assert!(clone_path.exists());
        assert!(clone_path.join(".git").exists());
    }
}
