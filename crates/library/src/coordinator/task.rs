use super::event::*;

use {
    kutil_std::error::*,
    std::path::*,
    tokio::{fs::*, sync::mpsc::*, task::*, *},
    tokio_util::sync::*,
};

/// Spawn coordinator task.
pub fn spawn_coordinator_task(
    coordinator_path: PathBuf,
    mut receiver: Receiver<Message>,
    cancellation: CancellationToken,
) -> JoinHandle<()> {
    tracing::info!("starting coordinator task: {}", coordinator_path.display());

    spawn(async move {
        // Touch on start
        touch(&coordinator_path).await;

        loop {
            select! {
                Some(message) = receiver.recv() => {
                    match message {
                        Ok(event) => {
                            if event.need_rescan() || event.kind.is_modify() {
                                if tracing::enabled!(tracing::Level::INFO) {
                                    for path in event.paths {
                                        tracing::info!("modified: {}", path.display());
                                    }
                                }

                                touch(&coordinator_path).await;
                            }
                        }

                        Err(error) => tracing::error!("{:?}: {}", error.kind, error),
                    }
                }

                _ = cancellation.cancelled() => {
                    tracing::info!("cancelled coordinator task: {}", coordinator_path.display());
                    break;
                }
            }
        }

        tracing::info!("coordinator task ended: {}", coordinator_path.display());

        // https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#clean-shutdown
        receiver.close();
    })
}

async fn touch(path: &PathBuf) {
    if let Err(error) = File::create(path).await.with_path(path) {
        tracing::error!("{}", error)
    }
}
