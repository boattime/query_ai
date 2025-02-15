use crate::metadata::CodeEntity;
use tree_sitter::Parser;

pub fn parse_rust(source_code: &str) -> Vec<CodeEntity> {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse code");

    extract_entities(&tree, source_code)
}

fn extract_entities(tree: &tree_sitter::Tree, source_code: &str) -> Vec<CodeEntity> {
    let root_node = tree.root_node();
    let mut entities = Vec::new();

    for node in root_node.children(&mut tree.walk()) {
        if node.kind() == "function_item" || node.kind() == "function_definition" {
            let name = node
                .child_by_field_name("name")
                .unwrap()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .to_string();
            entities.push(CodeEntity {
                name,
                kind: "function".to_string(),
                details: "".to_string(),
            });
        }
    }

    entities
}
