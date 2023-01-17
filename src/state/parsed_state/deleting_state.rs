use anyhow::{anyhow, Error};

/// Occurs when a temporary file is being deleted
pub enum DeletingState {
    DeletingTemporaryFile(String),
    ParseError(Error),
}

impl DeletingState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> DeletingState {
        let Some(next)=split.next() else { return DeletingState::ParseError( anyhow!("Deleting parse error"))};
        if next != "original" {
            return DeletingState::ParseError(anyhow!("Deleting parse error"));
        }
        let Some(next)=split.next() else { return DeletingState::ParseError( anyhow!("Deleting parse error"))};
        if next != "file" {
            return DeletingState::ParseError(anyhow!("Deleting parse error"));
        }
        split.next_back();
        split.next_back();
        split.next_back();
        split.next_back();
        let remaining = split.collect::<Vec<&str>>().join(" ");
        DeletingState::DeletingTemporaryFile(remaining)
    }
}
