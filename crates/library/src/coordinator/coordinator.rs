use {
    notify::*,
    std::{path::*, result::Result},
    tokio::{fs::*, sync::mpsc::*, task::*},
};

//
// Coordinator
//

/// File modification coordinator.
#[derive(Debug)]
pub struct Coordinator {
    /// [JoinHandle] for background task.
    pub background_task: JoinHandle<()>,

    watcher: notify::RecommendedWatcher,
}

impl Coordinator {
    /// Constructor.
    pub fn new(
        coordinator: PathBuf,
        follow_symlinks: bool,
        compare_contents: bool,
        channel_buffer_size: usize,
    ) -> Result<Self, Error> {
        let (sender, receiver) = channel(channel_buffer_size);

        let watcher = notify::RecommendedWatcher::new(
            SenderEventHandler(sender),
            notify::Config::default().with_follow_symlinks(follow_symlinks).with_compare_contents(compare_contents),
        )?;

        let background_task = spawn_background_task(coordinator, receiver);

        Ok(Self { background_task, watcher })
    }

    /// Wait for background task to end.
    pub async fn join_background_task(self) -> Result<(), JoinError> {
        self.background_task.await
    }

    /// Stop background task.
    pub fn stop_background_task(&self) {
        self.background_task.abort();
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

/// Channel message.
type Message = Result<Event, Error>;

/// Spawn background task.
fn spawn_background_task(coordinator: PathBuf, mut receiver: Receiver<Message>) -> JoinHandle<()> {
    tracing::info!("start background task");

    spawn(async move {
        while let Some(result) = receiver.recv().await {
            //tracing::debug!("received event");
            match result {
                Ok(event) => {
                    if let notify::EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            tracing::info!("modified: {}", path.display());
                        }

                        // "Touch" the coordinator file
                        if let Err(error) = File::create(&coordinator).await {
                            tracing::error!("{}", error)
                        }
                    }
                }

                Err(error) => tracing::error!("{}", error),
            }
        }

        tracing::info!("ending background task");
    })
}

//
// SenderEventHandler
//

/// Implements [EventHandler] for [Sender].
pub struct SenderEventHandler(Sender<Message>);

impl EventHandler for SenderEventHandler {
    fn handle_event(&mut self, event: Message) {
        //tracing::debug!("send event");
        let _ = self.0.blocking_send(event);
    }
}
