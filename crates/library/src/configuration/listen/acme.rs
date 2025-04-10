use super::super::error::*;

use {
    bytestring::*,
    compris::{normal::*, resolve::*},
    kutil_cli::debug::*,
    std::path::*,
};

//
// ACME
//

/// ACME.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct ACME {
    /// ACME directory URI.
    #[resolve]
    pub directory: Value,

    /// Contacts (usually email addresses).
    #[resolve]
    #[debuggable(iter(item), style(string))]
    pub contacts: Vec<ByteString>,

    /// Cache.
    #[resolve]
    #[debuggable(style(string))]
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
}
