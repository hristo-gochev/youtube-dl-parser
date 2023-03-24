/// Occurs when an youtube output is produced
pub enum YoutubeState {
    Initiating,
    ParseError(String),
}

impl YoutubeState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> YoutubeState {
        let Some(skipped)= split.next() else {return YoutubeState::ParseError("Unable to parse youtube state".to_owned())};
        let remaining = split.collect::<Vec<&str>>().join(" ");
        if remaining == "Downloading webpage" {
            YoutubeState::Initiating
        } else {
            YoutubeState::ParseError(format!("{skipped} {remaining}"))
        }
    }
}
