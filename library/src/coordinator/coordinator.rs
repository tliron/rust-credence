use super::{event::*, task::*};

use {
    notify::*,
    std::{path::*, result::Result},
    tokio::{sync::mpsc::*, task::*},
    tokio_util::sync::*,
};

//
// Coordinator
//

/// File modification coordinator.
#[derive(Debug)]
pub struct Coordinator {
    /// Watcher.
    pub watcher: RecommendedWatcher,

    /// [CancellationToken] for task.
    pub cancellation: CancellationToken,

    /// [JoinHandle] for task.
    pub task: JoinHandle<()>,
}

impl Coordinator {
    /// Constructor.
    pub fn new(
        coordinator_path: PathBuf,
        follow_symlinks: bool,
        compare_contents: bool,
        queue_size: usize,
    ) -> Result<Self, Error> {
        let (sender, receiver) = channel(queue_size);

        let watcher = RecommendedWatcher::new(
            SenderEventHandler(sender.clone()),
            Config::default().with_follow_symlinks(follow_symlinks).with_compare_contents(compare_contents),
        )?;

        let cancellation = CancellationToken::default();
        let task = spawn_coordinator_task(coordinator_path, receiver, cancellation.clone());

        Ok(Self { watcher, cancellation, task })
    }

    /// Shutdown (and wait for shutdown to complete).
    pub async fn shutdown(self) -> Result<(), JoinError> {
        self.cancellation.cancel();
        self.task.await
    }

    /// Add a coordinated path.
    ///
    /// If it's a directory it will be recursive.
    pub fn add<PathT>(&mut self, path: PathT) -> Result<(), Error>
    where
        PathT: AsRef<Path>,
    {
        let path = path.as_ref();
        self.watcher.watch(
            path,
            if path.is_dir() {
                tracing::info!("coordinating (recursively): {}", path.display());
                notify::RecursiveMode::Recursive
            } else {
                tracing::info!("coordinating: {}", path.display());
                notify::RecursiveMode::NonRecursive
            },
        )
    }

    /// Remove a coordinated path.
    #[allow(dead_code)]
    pub fn remove<PathT>(&mut self, path: PathT) -> Result<(), Error>
    where
        PathT: AsRef<Path>,
    {
        let path = path.as_ref();
        tracing::info!("no longer coordinating: {}", path.display());
        self.watcher.unwatch(path)
    }
}
