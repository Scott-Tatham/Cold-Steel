/*!
 * Stores the stream of events from the cargo command.
 */
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Child, ChildStderr, ChildStdout};
use super::structures::enums::event::Event;

/// Stores the stream of events from the cargo command.
#[derive(Debug)]
pub struct CommandEvents
{
    /// The stream of events from the cargo command.
    pub events: BufReader<ChildStdout>,

    /// The stream of errors from the cargo command.
    pub errors: BufReader<ChildStderr>
}

/// The definition of the [CommandEvents].
impl CommandEvents
{
    /// Initialises a new instance of the [CommandEvents].
    /// # Arguments
    /// * `command_child_process` - The child process of the command.
    /// # Returns
    /// `Result<CommandEvents, Error>` - The command events or an error.
    /// # Errors
    /// Returns an error if the stdout or stderr for the command cannot be read.
    pub fn new(command_child_process: &mut Child) -> Result<Self, Error>
    {
        let child_stdout = command_child_process.stdout.take();
        let child_stderr = command_child_process.stderr.take();

        match (child_stdout, child_stderr)
        {
            (Some(events), Some(errors)) =>
            {
                Ok(Self
                {
                    events: BufReader::new(events),
                    errors: BufReader::new(errors)
                })
            },
            (None, _) => Err(Error::new(ErrorKind::Other, "Error while reading stdout for the command.")),
            (_, None) => Err(Error::new(ErrorKind::Other, "Error while reading stderr for the command."))
        }
    }
}

/// The implementation of [Iterator] for [CommandEvents].
impl Iterator for CommandEvents
{
    /// The type of item returned by the iterator.
    type Item = Event;

    /// Returns the next event from the stream.
    /// # Returns
    /// `Option<Event>` - The next event or `None` if there are no more events.
    fn next(&mut self) -> Option<Self::Item>
    {
        let mut line = String::default();
        let remaining_lines = self.events.read_line(&mut line).ok();

        if remaining_lines? == 0
        {
            return None;
        }

        serde_json::from_str::<Event>(&line).ok()
    }
}