/// Occurs when an error is detected
pub enum ErrorState {
    Error(String),
}

impl ErrorState {
    pub fn parse<'a>(split: impl DoubleEndedIterator<Item = &'a str> + Send) -> ErrorState {
        let remaining = split.collect::<Vec<&str>>().join(" ");
        ErrorState::Error(remaining)
    }
}
