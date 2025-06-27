use super::super::error::*;

use {
    compris::resolve::*,
    kutil_cli::debug::*,
    kutil_http::tls::{ACME as KutilACME, *},
    kutil_std::zerocopy::*,
    std::path::*,
};

//
// ACME
//

/// TLS ACME.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct ACME {
    /// ACME directory URI.
    #[resolve]
    #[debuggable(style(string))]
    pub directory: ByteString,

    /// Contacts (usually email addresses).
    #[resolve(required)]
    #[debuggable(iter(item), style(string))]
    pub contacts: Vec<ByteString>,

    /// Cache.
    #[resolve]
    #[debuggable(as(debug), style(string))]
    pub cache: PathBuf,
}

impl ACME {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        if !self.cache.is_absolute() {
            self.cache = base_path.as_ref().join(&self.cache);
        }

        Ok(())
    }

    /// Create [ACME](KutilACME).
    pub fn provider(&self, host: ByteString) -> KutilACME {
        KutilACME {
            hosts: vec![host],
            directory: self.directory.clone(),
            contacts: self.contacts.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl Default for ACME {
    fn default() -> Self {
        Self { directory: LETS_ENCRYPT_STAGING_DIRECTORY.into(), contacts: Default::default(), cache: "acme".into() }
    }
}
