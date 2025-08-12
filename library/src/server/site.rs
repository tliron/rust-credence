use super::{
    super::{configuration::*, coordinator::*},
    configuration::*,
    routers::*,
};

use {axum::routing::*, kutil::http::axum::*, std::path::*};

//
// Site
//

/// Credence site.
#[derive(Clone, Debug)]
pub struct Site {
    /// Configuration.
    pub configuration: CredenceConfiguration,

    /// Router
    pub router: Router,
}

impl Site {
    /// Constructor.
    pub fn new<PathT>(assets_path: PathT, shutdown: &Shutdown) -> Result<Self, ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        let assets_path = assets_path.as_ref();

        if !assets_path.exists() {
            return Err(format!("assets path does not exist: {}", assets_path.display()).into());
        } else if !assets_path.is_dir() {
            return Err(format!("assets path is not a directory: {}", assets_path.display()).into());
        }

        let configuration = load_configuration(assets_path)?;
        let cache = configuration.caching.cache();
        let router = new_site_router(shutdown, &cache, &configuration);

        Ok(Self { configuration, router })
    }

    /// Create a [Coordinator] if configured.
    pub fn new_coordinator(&self) -> Result<Option<Coordinator>, ConfigurationError> {
        Ok(self.configuration.files.coordinate.new_coordinator()?)
    }
}
