use anyhow::{anyhow, Error};

/// Occurs when an youtube output is produced
pub enum YoutubeState {
    Initiating,
    ParseError(Error),
}

impl YoutubeState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> YoutubeState {
        let Some(skipped)= split.next() else {return YoutubeState::ParseError(anyhow!("Unable to parse youtube state"))};
        let remaining = split.collect::<Vec<&str>>().join(" ");
        if remaining == "Downloading webpage" {
            YoutubeState::Initiating
        } else {
            YoutubeState::ParseError(anyhow!("{skipped} {remaining}"))
        }
    }
}
