use clap::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ---------------------------------------

#[derive(Debug, Error)]
pub enum WorkerArgsError {
    #[error("config_file path length must be non-zero")]
    ConfigFilePathLengthMustBeNonZero,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WorkerArgs {
    /// Path to a .json configuration file
    #[arg(long)]
    pub config_file: String,
}

impl WorkerArgs {
    pub fn validate(&self) -> Result<(), WorkerArgsError> {
        if self.config_file.len() == 0 {
            return Err(WorkerArgsError::ConfigFilePathLengthMustBeNonZero);
        }
        Ok(())
    }
}

// ---------------------------------------

#[derive(Debug, Error)]
pub enum WorkerConfigError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error occurred: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub log_level: LogLevel,
    pub server_address: String,
}

impl WorkerConfig {
    pub fn from_file(file_path: String) -> Result<Self, WorkerConfigError> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);

        let config: Self = serde_json::from_reader(reader)?;
        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), WorkerConfigError> {
        Ok(())
    }

    pub fn log_level(&self) -> tracing::Level {
        match self.log_level {
            LogLevel::DEBUG => tracing::Level::DEBUG,
            LogLevel::INFO => tracing::Level::INFO,
            LogLevel::WARN => tracing::Level::WARN,
            LogLevel::ERROR => tracing::Level::ERROR,
        }
    }
}
