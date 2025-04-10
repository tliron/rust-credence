use super::{
    super::{render::*, util::*},
    coordinate::*,
    error::*,
};

use {compris::resolve::*, kutil_cli::debug::*, std::path::*};

//
// FilesConfiguration
//

/// Files configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct FilesConfiguration {
    /// Assets path.
    #[resolve]
    #[debuggable(style(string))]
    pub assets: PathBuf,

    /// Status path.
    #[resolve]
    #[debuggable(style(string))]
    pub status: PathBuf,

    /// Templates path.
    #[resolve]
    #[debuggable(style(string))]
    pub templates: PathBuf,

    /// Coordinate.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub coordinate: CoordinateConfiguration,
}

impl FilesConfiguration {
    /// Set assets paths if not already set.
    pub fn set_assets_path<PathT>(&mut self, assets_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        if self.assets.as_os_str().is_empty() {
            self.assets = assets_path.as_ref().into();
        }
    }

    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        let base_path = base_path.as_ref();

        if !self.status.is_absolute() {
            self.status = base_path.join(&self.status);
        }

        if !self.templates.is_absolute() {
            self.templates = base_path.join(&self.templates);
        }

        self.coordinate.validate(base_path, vec![self.templates.clone()])?;

        Ok(())
    }

    /// Asset path.
    pub fn asset(&self, uri_path: &str) -> PathBuf {
        self.assets.join(uri_path.trim_start_matches(PATH_SEPARATOR))
    }

    /// Templates.
    pub fn templates(&self) -> Templates {
        Templates::new(self)
    }
}

impl Default for FilesConfiguration {
    fn default() -> Self {
        Self {
            assets: PathBuf::default(),
            status: PathBuf::from("status"),
            templates: PathBuf::from("templates"),
            coordinate: CoordinateConfiguration::default(),
        }
    }
}
