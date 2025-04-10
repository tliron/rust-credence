use {kutil_cli::run::*, std::io, thiserror::*, tokio::task::*};

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

    /// Notify.
    #[error("notify: {0}")]
    Notify(#[from] notify::Error),
}

impl HasExit for MainError {
    fn get_exit(&self) -> Option<&Exit> {
        if let MainError::Exit(exit) = self { Some(exit) } else { None }
    }
}

//
// ConfigurationError
//

/// Configuration error.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Parse.
    #[error("parse: {0}")]
    Parse(#[from] compris::parse::ParseError),

    /// Resolve.
    #[error("resolve: {0}")]
    Resolve(#[from] compris::resolve::CommonResolveError),
}
