use crate::{Command, Message};

/// Causes the run loop to exit.
pub(crate) struct QuitMessage;

/// Pauses event processing. This can be useful when you spawn a new terminal application and your
/// application moves to the background. In this scenario you want to pause processing events in
/// your application to avoid conflicting with event processing the foreground application.
pub(crate) struct PauseEventsMessage;

/// Sent when `PauseEventsMessage` is successfully processed.
pub struct EventsPausedMessage;

/// Unpauses event processing. See `PauseEventsMessage`.
pub(crate) struct UnpauseEventsMessage;

/// Sent when `UnpauseEventsMessage` is successfully processed.
pub struct EventsUnpausedMessage;

/// A built in command that quits the application.
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

/// A built in command that pauses event processing.
pub fn pause_events() -> Option<Message> {
    Some(Box::new(PauseEventsMessage))
}

/// A built in command that unpauses event processing.
pub fn unpause_events() -> Option<Message> {
    Some(Box::new(UnpauseEventsMessage))
}

pub(crate) struct BatchMessage(pub Vec<Command>);

/// A built in command that combines multiple commands together.
///
/// These commands are executed in parallel, just like normal.
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}
