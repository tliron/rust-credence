use super::{super::coordinator::*, constants::*};

use {
    compris::resolve::*,
    httpdate::*,
    kutil_http::*,
    notify,
    std::{io, path::*},
};

//
// CoordinateConfiguration
//

/// File coordinator configuration.
#[derive(Clone, Debug, Resolve)]
pub struct CoordinateConfiguration {
    /// Paths to watch. When empty coordination will be disabled.
    #[resolve]
    pub paths: Vec<PathBuf>,

    /// Coordinator path. When [None](Option::None) coordination will be disabled.
    #[resolve]
    pub coordinator: Option<PathBuf>,

    /// Whether to follow symlinks.
    #[resolve(key = "follow-symlinks")]
    pub follow_symlinks: bool,

    /// Whether to compare contents.
    #[resolve(key = "compare-contents")]
    pub compare_contents: bool,

    /// Channel buffer size.
    #[resolve(key = "channel-buffer-size")]
    pub channel_buffer_size: usize,
}

impl CoordinateConfiguration {
    /// With default paths.
    pub fn with_default_paths(&mut self, default_paths: Vec<PathBuf>) {
        if self.paths.is_empty() {
            self.paths = default_paths;
        }
    }

    /// With base path.
    pub fn with_base_path<PathT>(&mut self, base_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        let base_path = base_path.as_ref();

        if let Some(coordinator) = &self.coordinator {
            if !coordinator.is_absolute() {
                self.coordinator = Some(base_path.join(coordinator.clone()));
            }
        }

        let len = self.paths.len();
        if len != 0 {
            let mut paths = Vec::with_capacity(len);
            for path in &self.paths {
                paths.push(if path.is_absolute() { path.clone() } else { base_path.join(path) });
            }
            self.paths = paths;
        }
    }

    /// Start.
    pub fn start(&self) -> notify::Result<Option<Coordinator>> {
        Ok(if let Some(coordinator) = &self.coordinator {
            if !self.paths.is_empty() {
                let mut coordinator = Coordinator::new(
                    coordinator.clone(),
                    self.follow_symlinks,
                    self.compare_contents,
                    self.channel_buffer_size,
                )?;

                for path in &self.paths {
                    coordinator.add(path)?;
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
            coordinator: Some(PathBuf::from(DEFAULT_COORDINATOR_FILE_NAME)),
            follow_symlinks: true,
            compare_contents: false,
            channel_buffer_size: 128,
        }
    }
}
