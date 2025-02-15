#[cfg(test)]
mod tests {
    use parser::process_repository;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn process_rust_repository_success() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();

        let main_rs_path = repo_path.join("src/main.rs");
        let lib_rs_path = repo_path.join("src/lib.rs");

        fs::create_dir_all(main_rs_path.parent().unwrap()).unwrap();

        fs::write(&main_rs_path, "fn hello() { println!(\"Hello, world!\"); }").unwrap();
        fs::write(&lib_rs_path, "pub fn add(a: i32, b: i32) -> i32 { a + b }").unwrap();

        let entities = process_repository(repo_path.to_str().unwrap());

        assert_eq!(entities.len(), 2);

        let hello_fn = entities.iter().find(|e| e.name == "hello").unwrap();
        assert_eq!(hello_fn.kind, "function");

        let add_fn = entities.iter().find(|e| e.name == "add").unwrap();
        assert_eq!(add_fn.kind, "function");
    }
}
