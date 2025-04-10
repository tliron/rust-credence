use {compris::resolve::*, std::path::*};

//
// TLS
//

/// TLS.
#[derive(Clone, Debug, Default, Resolve)]
pub struct TLS {
    /// Certificate PEM.
    #[resolve]
    pub certificate: LoadableBlob,

    /// Key PEM.
    #[resolve]
    pub key: LoadableBlob,
}

impl TLS {
    /// With base path.
    pub fn with_base_path<PathT>(&mut self, base_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        if let LoadableBlob::Path(path) = &mut self.certificate {
            if !path.is_absolute() {
                *path = base_path.as_ref().join(path.clone());
            }
        }

        if let LoadableBlob::Path(path) = &mut self.key {
            if !path.is_absolute() {
                *path = base_path.as_ref().join(path.clone());
            }
        }
    }
}
