# Query AI

A semantic code search system using RAG (Retrieval Augmented Generation), implemented in Rust.

## Overview

Query AI enables semantic code search and understanding of large codebases by combining code structure analysis with embedding-based retrieval. The system processes codebases to build both a structural graph representation and semantic embeddings, allowing for intelligent queries that understand code semantics.

## Features

- Semantic code search with natural language queries
- Code structure awareness through tree-sitter parsing
- Graph-based code relationship modeling with SurrealDB
- Vector search using Qdrant and CodeBERT embeddings
- CLI interface for indexing and querying

## Project Structure

```
code-rag/
├── Cargo.toml (workspace)
├── crates/
│   ├── parser/    - Tree-sitter integration for Rust parsing
│   ├── storage/   - SurrealDB and Qdrant integration
│   ├── embeddings/ - CodeBERT integration for vector generation
│   ├── core/      - Shared types and utilities
│   └── cli/       - Command-line interface
```

## Getting Started

### Prerequisites

- Rust toolchain (1.75+)
- SurrealDB
- Qdrant
- CodeBERT model

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/code-rag.git
   cd code-rag
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the CLI:
   ```
   cargo run --release -- --help
   ```

### Usage

#### Index a repository:

```
query_ai index https://github.com/example/repo.git
```

#### Query the code index:

```
query_ai query "How does the error handling work?"
```

#### Update the index from a webhook:

```
query_ai update webhook-payload.json
```

## Configuration

Create a `config.json` file in the project root or specify a configuration file with the `--config` option:

```json
{
  "logging": {
    "level": "info"
  },
  "parser": {
    "max_file_size": 1048576,
    "extensions": ["rs"],
    "exclude_dirs": ["target", ".git"]
  },
  "storage": {
    "surreal_url": "ws://localhost:8000",
    "surreal_user": "root",
    "surreal_password": "root",
    "qdrant_url": "http://localhost:6333",
    "qdrant_collection": "code-rag"
  },
  "embeddings": {
    "model_path": "models/codebert",
    "dimension": 768,
    "max_chunk_size": 512,
    "chunk_overlap": 128
  }
}
```

## Development

### Running Tests

```
cargo test
```

### Project Structure

- `core`: Common utilities, error types, and configuration
- `parser`: Code parsing and AST analysis
- `storage`: Database operations for graph and vector data
- `embeddings`: Embedding generation and management
- `cli`: Command-line interface
