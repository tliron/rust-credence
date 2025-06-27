use super::super::configuration::*;

use std::{fs::*, path::*};

/// [CredenceConfiguration].
pub fn load_configuration<PathT>(assets_path: PathT) -> Result<CredenceConfiguration, ConfigurationError>
where
    PathT: AsRef<Path>,
{
    let assets_path = assets_path.as_ref();
    let configuration_base_path = assets_path.join(CREDENCE_DIRECTORY_NAME);
    let configuration_path = configuration_base_path.join(CREDENCE_CONFIGURATION_FILE_NAME);

    let mut configuration = if configuration_path.exists() {
        CredenceConfiguration::read(
            &mut File::open(&configuration_path)?,
            configuration_path.to_string_lossy().into_owned().into(),
        )?
    } else {
        tracing::info!("configuration file not found: {}", configuration_path.display());
        Default::default()
    };

    configuration.files.set_assets_path(assets_path);
    configuration.validate(&configuration_base_path)?;

    Ok(configuration)
}
