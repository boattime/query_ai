use thiserror::Error;

/// The top-level error type for the query-ai system
#[derive(Error, Debug)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parse(#[from] ParseError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Embedding error: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// Error type for parsing operations
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse file {file}: {message}")]
    ParseFailed { file: String, message: String },

    #[error("Failed to load grammar: {0}")]
    GrammarLoadFailed(String),

    #[error("AST traversal error: {0}")]
    TraversalError(String),

    #[error("Invalid source code: {0}")]
    InvalidSource(String),
}

/// Error type for storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection error: {0}")]
    ConnectionFailed(String),

    #[error("Query execution error: {0}")]
    QueryFailed(String),

    #[error("Transaction error: {0}")]
    TransactionFailed(String),

    #[error("Data not found: {0}")]
    NotFound(String),

    #[error("Vector storage error: {0}")]
    VectorError(String),
}

/// Error type for embedding operations
#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Model loading error: {0}")]
    ModelLoadFailed(String),

    #[error("Embedding generation error: {0}")]
    GenerationFailed(String),

    #[error("Invalid input for embedding: {0}")]
    InvalidInput(String),

    #[error("Vector operation error: {0}")]
    VectorOperationFailed(String),
}

/// Error type for configuration operations
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration loading error: {0}")]
    LoadFailed(String),

    #[error("Missing configuration value: {0}")]
    MissingValue(String),

    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),

    #[error("Configuration validation error: {0}")]
    ValidationFailed(String),
}

/// Result type for code-rag operations
pub type Result<T> = std::result::Result<T, Error>;
