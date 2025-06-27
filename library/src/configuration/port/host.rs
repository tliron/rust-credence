use super::{super::error::*, acme::*, key::*};

use {compris::resolve::*, kutil_cli::debug::*, kutil_std::zerocopy::*, std::path::*};

//
// Host
//

/// Host.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct Host {
    /// Name.
    #[resolve(single)]
    #[debuggable(style(string))]
    pub name: ByteString,

    /// Whether to redirect all requests to this port.
    #[resolve(key = "redirect-to")]
    #[debuggable(option, style(number))]
    pub redirect_to: Option<u16>,

    /// Optional key configuration.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub key: Option<Key>,

    /// Optional ACME configuration.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub acme: Option<ACME>,
}

impl Host {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        if self.key.is_some() && self.acme.is_some() {
            return Err("host cannot have both `key` and `acme`".into());
        }

        let base_path = base_path.as_ref();

        if let Some(key) = &mut self.key {
            key.validate(base_path)?;
        }

        if let Some(acme) = &mut self.acme {
            acme.validate(base_path)?;
        }

        Ok(())
    }

    /// Whether we have TLS.
    pub fn has_tls(&self) -> bool {
        self.key.is_some() || self.acme.is_some()
    }
}
