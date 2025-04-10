use {kutil_http::tls::*, std::io, thiserror::*};

//
// ConfigurationError
//

/// Configuration error.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// TLS.
    #[error("TLS: {0}")]
    TLS(#[from] TlsContainerError),

    /// Notify.
    #[error("notify: {0}")]
    Notify(#[from] notify::Error),

    /// Validation.
    #[error("validation: {0}")]
    Validation(String),
}

impl From<String> for ConfigurationError {
    fn from(message: String) -> Self {
        Self::Validation(message)
    }
}

impl From<&str> for ConfigurationError {
    fn from(message: &str) -> Self {
        Self::Validation(message.into())
    }
}
