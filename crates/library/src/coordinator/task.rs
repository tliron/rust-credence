use super::event::*;

use {
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
        loop {
            select! {
                Some(message) = receiver.recv() => {
                    match message {
                        Ok(event) => {
                            if let notify::EventKind::Modify(_) = event.kind {
                                for path in event.paths {
                                    tracing::info!("modified: {}", path.display());
                                }

                                // "Touch" the coordinator file
                                if let Err(error) = File::create(&coordinator_path).await {
                                    tracing::error!("{}", error)
                                }
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
