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

    /// Key store.
    #[error("key store: {0}")]
    KeyStore(#[from] TlsProviderError),

    /// Custom.
    #[error("{0}")]
    Custom(String),
}

impl From<String> for ConfigurationError {
    fn from(message: String) -> Self {
        Self::Custom(message)
    }
}

impl From<&str> for ConfigurationError {
    fn from(message: &str) -> Self {
        Self::Custom(message.into())
    }
}
