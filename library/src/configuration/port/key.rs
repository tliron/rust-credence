use super::{super::error::*, loadable_bytes::*};

use {
    compris::resolve::*,
    kutil::{cli::depict::*, std::immutable::*},
    std::path::*,
};

//
// Key
//

/// TLS key.
#[derive(Clone, Debug, Default, Depict, Resolve)]
pub struct Key {
    /// Certificates PEM.
    #[resolve(required)]
    #[depict(as(display), style(symbol))]
    pub certificates: LoadableBytes,

    /// Private key PEM.
    #[resolve(required, key = "private-key")]
    #[depict(as(display), style(symbol))]
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
