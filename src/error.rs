use thiserror::Error;
use winit::error::{EventLoopError, OsError};

#[derive(Debug, Error)]
pub enum PixuError {
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    #[error("OS error: {0}")]
    OsError(#[from] OsError),
}

pub type PixuResult<T> = Result<T, PixuError>;
