use std::str::FromStr;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::error::{ConfigError, Error, Result};

/// Log levels supported by the application
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl FromStr for LogLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(Error::Config(ConfigError::InvalidValue(format!(
                "Invalid log level: {}",
                s
            )))),
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

/// Initialize the logging system with the specified level
pub fn init(level: LogLevel) -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::from(level).into());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .try_init()
        .map_err(|e| Error::Other(format!("Failed to initialize logging: {}", e)))?;

    tracing::info!("Logging initialized at level: {:?}", level);
    Ok(())
}

/// Initialize the logging system with default settings (INFO level)
pub fn init_default() -> Result<()> {
    init(LogLevel::Info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert!(matches!(
            LogLevel::from_str("trace").unwrap(),
            LogLevel::Trace
        ));
        assert!(matches!(
            LogLevel::from_str("debug").unwrap(),
            LogLevel::Debug
        ));
        assert!(matches!(
            LogLevel::from_str("info").unwrap(),
            LogLevel::Info
        ));
        assert!(matches!(
            LogLevel::from_str("warn").unwrap(),
            LogLevel::Warn
        ));
        assert!(matches!(
            LogLevel::from_str("error").unwrap(),
            LogLevel::Error
        ));

        assert!(LogLevel::from_str("invalid").is_err());
    }
}
