use anyhow::Error;

/// Raw output state
pub enum OutputState {
    /// Occurs when a raw reader reads
    Outputting(String),
    /// Occurs when the output reader is finished
    Finished,
    /// Occurs when there's an error reading and the reader ends unexpectedly
    Error(Error),
}
