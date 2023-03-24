/// Occurs when a temporary file is being deleted
pub enum DeletingState {
    DeletingTemporaryFile(String),
    ParseError(String),
}

impl DeletingState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> DeletingState {
        let Some(next)=split.next() else { return DeletingState::ParseError( "Deleting parse error".to_owned())};
        if next != "original" {
            return DeletingState::ParseError("Deleting parse error".to_owned());
        }
        let Some(next)=split.next() else { return DeletingState::ParseError( "Deleting parse error".to_owned())};
        if next != "file" {
            return DeletingState::ParseError("Deleting parse error".to_owned());
        }
        split.next_back();
        split.next_back();
        split.next_back();
        split.next_back();
        let remaining = split.collect::<Vec<&str>>().join(" ");
        DeletingState::DeletingTemporaryFile(remaining)
    }
}
