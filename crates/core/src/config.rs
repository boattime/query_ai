use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::error::{ConfigError, Error, Result};
use crate::logging::LogLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,

    pub parser: ParserConfig,

    pub storage: StorageConfig,

    pub embeddings: EmbeddingsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub max_file_size: usize,

    pub extensions: Vec<String>,

    pub exclude_dirs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub surreal_url: String,

    pub surreal_user: String,

    pub surreal_password: String,

    pub qdrant_url: String,

    pub qdrant_collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub model_path: String,

    pub dimension: usize,

    pub max_chunk_size: usize,

    pub chunk_overlap: usize,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let config_path = path.as_ref();

        if !config_path.exists() {
            return Err(Error::Config(ConfigError::LoadFailed(format!(
                "Configuration file not found: {}",
                config_path.display()
            ))));
        }

        let config = ConfigLoader::builder()
            .add_source(File::from(config_path))
            .build()
            .map_err(|e| Error::Config(ConfigError::LoadFailed(e.to_string())))?
            .try_deserialize()
            .map_err(|e| Error::Config(ConfigError::LoadFailed(e.to_string())))?;

        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        let config_paths = vec![
            PathBuf::from("config.json"),
            PathBuf::from("config/config.json"),
            PathBuf::from("/etc/query-ai/config.json"),
        ];

        for path in config_paths {
            if path.exists() {
                return Self::load(path);
            }
        }

        Ok(Self::default())
    }

    pub fn validate(&self) -> Result<()> {
        let _ = LogLevel::from_str(&self.logging.level)?;

        if self.storage.surreal_url.is_empty() {
            return Err(Error::Config(ConfigError::ValidationFailed(
                "SurrealDB URL cannot be empty".to_string(),
            )));
        }

        if self.storage.qdrant_url.is_empty() {
            return Err(Error::Config(ConfigError::ValidationFailed(
                "Qdrant URL cannot be empty".to_string(),
            )));
        }

        if self.embeddings.dimension == 0 {
            return Err(Error::Config(ConfigError::ValidationFailed(
                "Embedding dimension must be greater than 0".to_string(),
            )));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logging: LoggingConfig {
                level: "info".to_string(),
            },
            parser: ParserConfig {
                max_file_size: 1024 * 1024,
                extensions: vec!["rs".to_string()],
                exclude_dirs: vec!["target".to_string(), ".git".to_string()],
            },
            storage: StorageConfig {
                surreal_url: "ws://localhost:8000".to_string(),
                surreal_user: "root".to_string(),
                surreal_password: "root".to_string(),
                qdrant_url: "http://localhost:6333".to_string(),
                qdrant_collection: "code-rag".to_string(),
            },
            embeddings: EmbeddingsConfig {
                model_path: "models/codebert".to_string(),
                dimension: 768,
                max_chunk_size: 512,
                chunk_overlap: 128,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.parser.max_file_size, 1024 * 1024);
        assert!(config.parser.extensions.contains(&"rs".to_string()));
    }

    #[test]
    fn test_load_config() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("config.json");

        let config_json = r#"{
            "logging": {
                "level": "debug"
            },
            "parser": {
                "max_file_size": 2097152,
                "extensions": ["rs", "toml"],
                "exclude_dirs": ["target", ".git", "node_modules"]
            },
            "storage": {
                "surreal_url": "ws://localhost:8000",
                "surreal_user": "testuser",
                "surreal_password": "testpass",
                "qdrant_url": "http://localhost:6333",
                "qdrant_collection": "test-collection"
            },
            "embeddings": {
                "model_path": "models/test-model",
                "dimension": 512,
                "max_chunk_size": 256,
                "chunk_overlap": 64
            }
        }"#;

        std::fs::write(&file_path, config_json).unwrap();

        let config = Config::load(&file_path).unwrap();
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.parser.max_file_size, 2097152);
        assert!(config.parser.extensions.contains(&"rs".to_string()));
        assert!(config.parser.extensions.contains(&"toml".to_string()));
        assert_eq!(config.embeddings.dimension, 512);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
        config.logging.level = "info".to_string();

        config.storage.surreal_url = "".to_string();
        assert!(config.validate().is_err());
        config.storage.surreal_url = "ws://localhost:8000".to_string();

        config.embeddings.dimension = 0;
        assert!(config.validate().is_err());
    }
}
