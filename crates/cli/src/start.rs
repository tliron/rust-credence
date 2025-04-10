use credence_lib::configuration::ConfigurationError;

use super::{cli::*, errors::*};

use {
    credence_lib::server::*,
    kutil_cli::debug::*,
    kutil_http::axum::*,
    std::{io, time::*},
};

impl CLI {
    /// Start.
    pub async fn start(&self) -> Result<(), MainError> {
        let shutdown = self.shutdown()?;
        let sites = self.sites(&shutdown)?;

        // Servers
        let mut servers = Servers::new(shutdown.handle);
        for site in &sites {
            servers.add_site(site)?;
        }

        // Serve!
        let join_set = servers.start()?;
        if join_set.is_empty() {
            return Err(ConfigurationError::from("no servers").into());
        }

        // Coordinators
        let mut coordinators = Vec::new();
        for site in &sites {
            if let Some(coordinator) = site.new_coordinator()? {
                coordinators.push(coordinator);
            }
        }

        // Wait for servers to shutdown
        for result in join_set.join_all().await {
            if let Err(error) = result {
                tracing::error!("{}", error);
            }
        }

        // Shutdown coordinators
        for coordinator in coordinators {
            if let Err(error) = coordinator.shutdown().await {
                tracing::error!("{}", error);
            }
        }

        tracing::info!("goodbye");

        Ok(())
    }

    /// Sites.
    pub fn sites(&self, shutdown: &Shutdown) -> Result<Vec<Site>, MainError> {
        let mut sites = Vec::new();

        for assets_path in &self.assets_paths {
            let site = Site::new(assets_path, &shutdown)?;

            tracing::info!("added site: {}", assets_path.display());
            if !self.quiet && (self.verbose > 0) {
                site.configuration.eprint_debug();
            }

            sites.push(site);
        }

        Ok(sites)
    }

    /// [Shutdown].
    pub fn shutdown(&self) -> io::Result<Shutdown> {
        let shutdown = Shutdown::new(Some(Duration::from_secs(self.grace_period)));
        shutdown.on_signals()?;
        Ok(shutdown)
    }
}
