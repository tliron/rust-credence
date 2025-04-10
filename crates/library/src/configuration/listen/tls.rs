use super::{super::error::*, acme::*, loadable_bytes::*};

use {
    compris::resolve::*,
    kutil_cli::debug::*,
    std::{io, path::*},
};

//
// TLS
//

/// TLS.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct TLS {
    /// Certificates PEM.
    #[resolve]
    #[debuggable(option, as(display), style(bare))]
    pub certificates: Option<LoadableBytes>,

    /// Private key PEM.
    #[resolve(key = "private-key")]
    #[debuggable(option, as(display), style(bare))]
    pub private_key: Option<LoadableBytes>,

    /// ACME.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub acme: Option<ACME>,
}

impl TLS {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        let has_certificates = self.certificates.is_some();
        let has_private_key = self.private_key.is_some();
        let has_acme = self.acme.is_some();

        if !has_certificates && !has_private_key && !has_acme {
            return Err("`tls`: must set either `certificates`/`private-key` or `acme`: {}".into());
        }

        if (has_certificates || has_private_key) && has_acme {
            return Err("`tls`: cannot set both `certificates`/`private-key` and `acme`".into());
        }

        if (has_certificates && !has_private_key) || (has_private_key && !has_certificates) {
            return Err("`tls`: `certificates` and `private-key` must be set together".into());
        }

        if let Some(LoadableBytes::Path(path)) = &mut self.certificates {
            if !path.is_absolute() {
                *path = base_path.as_ref().join(&path);
            }
        }

        if let Some(LoadableBytes::Path(path)) = &mut self.private_key {
            if !path.is_absolute() {
                *path = base_path.as_ref().join(&path);
            }
        }

        if let Some(acme) = &mut self.acme {
            acme.validate(base_path)?;
        }

        Ok(())
    }

    /// Ensures both certificates and private key are loaded.
    pub fn into_bytes(self) -> io::Result<Self> {
        Ok(Self {
            certificates: match self.certificates {
                Some(certificates) => Some(certificates.into_bytes()?),
                None => None,
            },
            private_key: match self.private_key {
                Some(private_key) => Some(private_key.into_bytes()?),
                None => None,
            },
            acme: self.acme,
        })
    }
}
