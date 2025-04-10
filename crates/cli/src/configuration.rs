use super::cli::*;

use {
    compris::{parse::Parser, resolve::*, *},
    credence_lib::configuration::*,
    std::{fs::*, io},
};

impl CLI {
    /// Server configuration.
    pub fn server_configuration(&self) -> io::Result<ServerConfiguration> {
        let assets_path = self.path.canonicalize()?;
        let configuration_base_path = assets_path.join(CREDENCE_DIRECTORY_NAME);
        let configuration_path = configuration_base_path.join(CREDENCE_CONFIGURATION_FILE_NAME);

        let mut configuration = if configuration_path.exists() {
            let value = Parser::new(Format::YAML)
                .with_try_unsigned_integers(true)
                .parse(&mut File::open(configuration_path)?)
                .map_err(io::Error::other)?;

            <normal::Value as Resolve<_, CommonResolveContext, CommonResolveError>>::resolve(&value)
                .map_err(io::Error::other)?
                .ok_or(io::Error::other("no configuration"))?
        } else {
            tracing::info!("configuration not found: {}", configuration_path.display());
            ServerConfiguration::default()
        };

        configuration.paths.with_assets_path(assets_path);
        configuration.with_base_path(&configuration_base_path);

        Ok(configuration)
    }
}
