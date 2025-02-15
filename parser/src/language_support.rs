use crate::ast_extractor::parse_rust;
use crate::metadata::CodeEntity;

pub fn parse_code(_file_path: &str, content: &str) -> Vec<CodeEntity> {
    parse_rust(content)
}
