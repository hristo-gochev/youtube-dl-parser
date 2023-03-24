/// Occurs when ffmpeg is being used
pub enum FFmpegState {
    Destination(String),
    ParseError(String),
}

impl FFmpegState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> FFmpegState {
        let Some(first_word)= split.next() else { return FFmpegState::ParseError("Unable to get destination of mp3 file".to_owned())};
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
                FFmpegState::ParseError(format!("{first_word} {leftover}"))
            }
        }
    }
}
