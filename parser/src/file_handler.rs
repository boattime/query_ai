use std::fs;
use walkdir::WalkDir;

pub fn load_repository_files(repo_dir: &str) -> Vec<(String, String)> {
    let mut files = Vec::new();

    for entry in WalkDir::new(repo_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                if let Ok(content) = fs::read_to_string(path) {
                    files.push((path.to_string_lossy().to_string(), content));
                }
            }
        }
    }

    files
}
