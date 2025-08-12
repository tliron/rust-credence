use {credence_lib::configuration::*, kutil::cli::run::*, std::io, thiserror::*, tokio::task::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Configuration.
    #[error("configuration: {0}")]
    Configuration(#[from] ConfigurationError),

    /// Join.
    #[error("join: {0}")]
    Join(#[from] JoinError),
}

impl RunError for MainError {
    fn handle(&self) -> (bool, u8) {
        match self {
            MainError::Configuration(error) => (error.eprint_validation_errors(), 1),
            _ => (false, 1),
        }
    }
}
