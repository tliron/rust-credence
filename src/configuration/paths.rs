use super::{super::render::*, constants::*};

use {compris::resolve::*, std::path::*};

//
// PathsConfiguration
//

/// Paths configuration.
#[derive(Clone, Debug, Resolve)]
pub struct PathsConfiguration {
    /// Assets path.
    #[resolve]
    pub assets: PathBuf,

    /// Status path.
    #[resolve]
    pub status: PathBuf,

    /// Templates path.
    #[resolve]
    pub templates: PathBuf,
}

impl PathsConfiguration {
    /// With assets path.
    pub fn with_assets_path<PathT>(&mut self, assets_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        if self.assets.as_os_str().is_empty() {
            self.assets = assets_path.as_ref().into();
        }
    }

    /// With base path.
    pub fn with_base_path<PathT>(&mut self, base_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        let base_path = base_path.as_ref();

        if !self.status.is_absolute() {
            self.status = base_path.join(self.status.clone());
        }

        if !self.templates.is_absolute() {
            self.templates = base_path.join(self.templates.clone());
        }
    }

    /// Asset path.
    pub fn asset(&self, uri_path: &str) -> PathBuf {
        self.assets
            .join(uri_path.trim_start_matches(PATH_SEPARATOR))
    }

    /// Templates.
    pub fn templates(&self) -> Templates {
        Templates::new(&self.templates)
    }
}

impl Default for PathsConfiguration {
    fn default() -> Self {
        Self {
            assets: PathBuf::default(),
            status: PathBuf::from("status"),
            templates: PathBuf::from("templates"),
        }
    }
}
