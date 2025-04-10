use super::{cli::*, configuration::*, errors::*};

use {kutil_cli::run::*, tokio::task::*};

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

        for listen in configuration.listen {
            for server in listen.axum_servers(&router).await? {
                tasks.spawn(server);
            }
        }

        if tasks.is_empty() {
            tracing::warn!("no listeners specified or found");
        }

        tracing::info!("assets: {:?}", self.path);

        tasks.join_all().await;

        Ok(())
    }
}
