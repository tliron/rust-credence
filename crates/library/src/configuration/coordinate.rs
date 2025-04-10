use super::{super::coordinator::*, error::*};

use {
    compris::resolve::*,
    httpdate::*,
    kutil_cli::debug::*,
    kutil_http::*,
    notify,
    std::{io, path::*},
};

//
// CoordinateConfiguration
//

/// File coordinator configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct CoordinateConfiguration {
    /// Paths to watch. When empty coordination will be disabled.
    #[resolve]
    #[debuggable(iter(item), style(string))]
    pub paths: Vec<PathBuf>,

    /// Coordinator path. When [None](Option::None) coordination will be disabled.
    #[resolve]
    #[debuggable(option, style(string))]
    pub coordinator: Option<PathBuf>,

    /// Whether to follow symlinks.
    #[resolve(key = "follow-symlinks")]
    #[debuggable(style(symbol))]
    pub follow_symlinks: bool,

    /// Whether to compare contents.
    #[resolve(key = "compare-contents")]
    #[debuggable(style(symbol))]
    pub compare_contents: bool,

    /// Message queue size.
    #[resolve(key = "queue-size")]
    #[debuggable(style(number))]
    pub queue_size: usize,
}

impl CoordinateConfiguration {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT, default_paths: Vec<PathBuf>) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        let base_path = base_path.as_ref();

        if let Some(coordinator) = &self.coordinator {
            if !coordinator.is_absolute() {
                self.coordinator = Some(base_path.join(&coordinator));
            }
        }

        if self.paths.is_empty() {
            self.paths = default_paths;
        } else {
            let mut paths = Vec::with_capacity(self.paths.len());
            for path in &self.paths {
                paths.push(if path.is_absolute() { path.clone() } else { base_path.join(path) });
            }
            self.paths = paths;
        }

        Ok(())
    }

    /// Construct a [Coordinator] if configured.
    pub fn new_coordinator(&self) -> notify::Result<Option<Coordinator>> {
        Ok(if let Some(coordinator) = &self.coordinator {
            if !self.paths.is_empty() {
                let mut coordinator = Coordinator::new(
                    coordinator.clone(),
                    self.follow_symlinks,
                    self.compare_contents,
                    self.queue_size,
                )?;

                for path in &self.paths {
                    if path.exists() {
                        coordinator.add(path)?;
                    } else {
                        tracing::info!("path does not exist: {}", path.display());
                    }
                }

                Some(coordinator)
            } else {
                None
            }
        } else {
            None
        })
    }

    /// Coordinator modified timestamp.
    pub fn coordinator_modified(&self) -> io::Result<Option<HttpDate>> {
        Ok(match &self.coordinator {
            Some(coordinator) => {
                if coordinator.exists() {
                    Some(file_modified(coordinator)?)
                } else {
                    None
                }
            }
            None => None,
        })
    }
}

impl Default for CoordinateConfiguration {
    fn default() -> Self {
        Self {
            paths: Vec::default(),
            coordinator: Some(PathBuf::from(".coordinator")),
            follow_symlinks: true,
            compare_contents: false,
            queue_size: 128,
        }
    }
}
