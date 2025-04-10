use super::{cli::*, errors::*};

use {credence_lib::configuration::*, kutil_cli::run::*, tokio::task::*};

impl CLI {
    pub async fn serve(&self) -> Result<(), MainError> {
        if !self.path.exists() {
            return Err(Exit::new(1, Some("web assets path does not exist")).into());
        } else if !self.path.is_dir() {
            return Err(Exit::new(1, Some("web assets path is not a directory")).into());
        }

        let configuration = self.server_configuration()?;

        tracing::info!("{:#?}", configuration);

        let cache = configuration.caching.cache();
        let router = self.router(cache, &configuration).into_make_service();

        let mut tasks = JoinSet::new();

        // Make sure to bind to a variable so it won't drop (and stop)
        let coordinator = configuration.paths.coordinate.start()?;

        for listen in configuration.listen {
            for server in listen.axum_servers(&router).await? {
                tasks.spawn(server);
            }
        }

        if tasks.is_empty() {
            tracing::warn!("no listeners specified or found");
        }

        tracing::info!("assets: {:?}", self.path);

        // Join

        for result in tasks.join_all().await {
            result?;
        }

        if let Some(coordinator) = coordinator {
            coordinator.join_background_task().await?;
        }

        tracing::info!("stopped");

        Ok(())
    }
}
