pub mod ast_extractor;
pub mod file_handler;
pub mod language_support;
pub mod metadata;

use file_handler::load_repository_files;
use language_support::parse_code;
use metadata::CodeEntity;

pub fn process_repository(repo_file: &str) -> Vec<CodeEntity> {
    let files = load_repository_files(repo_file);
    let mut entities = Vec::new();

    for (file_path, content) in files {
        let parsed_entities = parse_code(&file_path, &content);
        entities.extend(parsed_entities);
    }

    entities
}
