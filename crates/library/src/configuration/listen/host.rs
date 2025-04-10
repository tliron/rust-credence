use super::{super::error::*, tls::*};

use {bytestring::*, compris::resolve::*, kutil_cli::debug::*, std::path::*};

//
// Host
//

/// Host.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct Host {
    /// Host.
    #[resolve(single)]
    #[debuggable(style(string))]
    pub host: ByteString,

    /// Optional TLS configuration.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub tls: Option<TLS>,
}

impl Host {
    /// Constructor.
    pub fn new(host: ByteString, tls: Option<TLS>) -> Self {
        Self { host, tls }
    }

    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        if let Some(mut tls) = self.tls.take() {
            tls.validate(base_path)?;
            self.tls = Some(tls.into_bytes()?);
        }

        Ok(())
    }
}
