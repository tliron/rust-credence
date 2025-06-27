use super::{super::error::*, loadable_bytes::*};

use {compris::resolve::*, kutil_cli::debug::*, kutil_std::zerocopy::*, std::path::*};

//
// Key
//

/// TLS key.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct Key {
    /// Certificates PEM.
    #[resolve(required)]
    #[debuggable(as(display), style(symbol))]
    pub certificates: LoadableBytes,

    /// Private key PEM.
    #[resolve(required, key = "private-key")]
    #[debuggable(as(display), style(symbol))]
    pub private_key: LoadableBytes,
}

impl Key {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        if let LoadableBytes::Path(path) = &mut self.certificates
            && !path.is_absolute()
        {
            *path = base_path.as_ref().join(&path);
        }

        if let LoadableBytes::Path(path) = &mut self.private_key
            && !path.is_absolute()
        {
            *path = base_path.as_ref().join(&path);
        }

        Ok(())
    }

    /// Ensures both certificates and private key are loaded.
    pub fn to_bytes(&self) -> Result<(Bytes, Bytes), ConfigurationError> {
        Ok((self.certificates.to_bytes()?, self.private_key.to_bytes()?))
    }
}
