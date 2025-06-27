use {notify::*, std::result::Result, tokio::sync::mpsc::*};

/// Channel message.
pub type Message = Result<Event, Error>;

//
// SenderEventHandler
//

/// Implements [EventHandler] for [Sender].
pub struct SenderEventHandler(pub Sender<Message>);

impl EventHandler for SenderEventHandler {
    fn handle_event(&mut self, event: Message) {
        if let Err(error) = self.0.blocking_send(event) {
            tracing::error!("{}", error);
        }
    }
}
