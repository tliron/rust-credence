use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::depict::*, http::tls::*},
    std::io,
    thiserror::*,
};

//
// ConfigurationError
//

/// Configuration error.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    /// None.
    #[error("no configuration")]
    None,

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
    Validation(ResolveErrors<WithAnnotations>),
}

impl ConfigurationError {
    /// Prints validation errors.
    pub fn eprint_validation_errors(&self) -> bool {
        match self {
            Self::Validation(errors) => {
                errors.annotated_depictions(Some("Invalid CredenceConfiguration".into())).eprint_default_depiction();
                true
            }

            _ => false,
        }
    }
}

impl From<ResolveErrors<WithAnnotations>> for ConfigurationError {
    fn from(errors: ResolveErrors<WithAnnotations>) -> Self {
        Self::Validation(errors)
    }
}

impl From<String> for ConfigurationError {
    fn from(message: String) -> Self {
        let error: ResolveError<_> = message.into();
        Self::Validation(error.into())
    }
}

impl From<&str> for ConfigurationError {
    fn from(message: &str) -> Self {
        String::from(message).into()
    }
}
