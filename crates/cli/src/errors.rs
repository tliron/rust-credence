use {credence_lib::configuration::*, kutil_cli::run::*, std::io, thiserror::*, tokio::task::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    /// Exit.
    #[error("exit: {0}")]
    Exit(#[from] Exit),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Configuration.
    #[error("configuration: {0}")]
    Configuration(#[from] ConfigurationError),

    /// Join.
    #[error("notify: {0}")]
    Join(#[from] JoinError),
}

impl HasExit for MainError {
    fn get_exit(&self) -> Option<&Exit> {
        if let MainError::Exit(exit) = self { Some(exit) } else { None }
    }
}
