use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Audio output unavailable: {0}")]
    NoOutput(String),
    #[error("Sample not found: {0}")]
    SampleNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("{0}")]
    Audio(#[from] AudioError),
    #[error("{0}")]
    Project(#[from] ProjectError),
}

pub type AudioResult<T> = Result<T, AudioError>;
pub type ProjectResult<T> = Result<T, ProjectError>;
