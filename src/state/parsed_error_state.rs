/// Occurs when there's an error reading and the reader ends unexpectedly
pub struct ParsedErrorState {
    pub error: String,
    pub exit_code: u32,
}

impl ParsedErrorState {
    pub fn parse(error: String) -> Self {
        let mut split = error.split(' ').filter(|string| !string.is_empty());
        let exit_code = split
            .next_back()
            .and_then(|exit_code| exit_code.parse::<u32>().ok())
            .unwrap_or(1);
        Self { error, exit_code }
    }
}
