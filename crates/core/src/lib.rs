//! Core crate for the query-ai system
//!
//! This crate provides common utilities, error types, logging,
//! and configuration management for the code-rag system.

pub mod config;
pub mod error;
pub mod logging;

// Re-export commonly used types
pub use config::Config;
pub use error::{Error, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
