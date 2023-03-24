mod deleting_state;
mod download_state;
mod error_state;
mod ffmpeg_state;
mod youtube_state;

pub use crate::state::parsed_state::deleting_state::DeletingState;
pub use crate::state::parsed_state::download_state::DownloadState;
pub use crate::state::parsed_state::error_state::ErrorState;
pub use crate::state::parsed_state::ffmpeg_state::FFmpegState;
pub use crate::state::parsed_state::youtube_state::YoutubeState;

enum ParsedStateHeader {
    YouTube,
    Download,
    FFmpeg,
    Deleting,
    Error,
    Other(String),
}

/// General parsed state, can contain all of the more specific states
pub enum ParsedState {
    Youtube(YoutubeState),
    Download(DownloadState),
    FFMpeg(FFmpegState),
    Error(ErrorState),
    Deleting(DeletingState),
    None(String),
    Unknown(String),
    ParseError(String),
}

impl ParsedState {
    pub fn parse(output: String) -> Self {
        let mut split = output.split(' ').filter(|string| !string.is_empty());
        let Some(state_header)= split.next() else { return ParsedState::ParseError("Output is empty".to_owned()) };
        let Some(state_header) = Self::parse_state_header(state_header) else { return ParsedState::None(output)};
        match state_header {
            ParsedStateHeader::YouTube => ParsedState::Youtube(YoutubeState::parse(split)),
            ParsedStateHeader::Download => ParsedState::Download(DownloadState::parse(split)),
            ParsedStateHeader::FFmpeg => ParsedState::FFMpeg(FFmpegState::parse(split)),
            ParsedStateHeader::Deleting => ParsedState::Deleting(DeletingState::parse(split)),
            ParsedStateHeader::Error => ParsedState::Error(ErrorState::parse(split)),
            ParsedStateHeader::Other(_) => ParsedState::Unknown(output),
        }
    }

    // Parse header
    fn parse_state_header(state_header: &str) -> Option<ParsedStateHeader> {
        let mut chars = state_header.chars();
        let first_char = chars.next()?;
        let last_char = chars.next_back()?;
        if first_char == '[' && last_char == ']' {
            let mode = chars.as_str();
            match mode {
                "youtube" => Some(ParsedStateHeader::YouTube),
                "download" => Some(ParsedStateHeader::Download),
                "ffmpeg" => Some(ParsedStateHeader::FFmpeg),
                _ => Some(ParsedStateHeader::Other(mode.to_owned())),
            }
        } else if state_header == "ERROR:" {
            Some(ParsedStateHeader::Error)
        } else if state_header == "Deleting" {
            Some(ParsedStateHeader::Deleting)
        } else {
            None
        }
    }
}
