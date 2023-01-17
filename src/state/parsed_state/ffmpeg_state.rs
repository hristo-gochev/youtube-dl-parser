use anyhow::{anyhow, Error};

/// Occurs when ffmpeg is being used
pub enum FFmpegState {
    Destination(String),
    ParseError(Error),
}

impl FFmpegState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> FFmpegState {
        let Some(first_word)= split.next() else { return FFmpegState::ParseError(anyhow!("Unable to get destination of mp3 file"))};
        match first_word {
            "Destination:" => {
                let destination = split.collect::<Vec<&str>>().join(" ");
                FFmpegState::Destination(destination)
            }
            "Merging" => {
                split.next();
                split.next();
                let destination = split.collect::<Vec<&str>>().join(" ");
                FFmpegState::Destination(destination)
            }
            _ => {
                let leftover = split.collect::<Vec<&str>>().join(" ");
                FFmpegState::ParseError(anyhow!("{first_word} {leftover}"))
            }
        }
    }
}
