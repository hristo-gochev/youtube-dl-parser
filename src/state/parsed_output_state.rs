use crate::state::output_state::OutputState;
use crate::state::parsed_error_state::ParsedErrorState;
use crate::state::parsed_state::ParsedState;

/// The main state that a parsed output reader produces
pub enum ParsedOutputState {
    /// Occurs when an output was detected and parsed
    Parsed(ParsedState),
    /// Occurs when the output reader is finished
    Finished,
    /// Occurs when there's an error reading and the reader ends unexpectedly
    Error(ParsedErrorState),
}

impl ParsedOutputState {
    /// Parses a raw output state into a more specific one
    pub fn parse(output_state: OutputState) -> ParsedOutputState {
        match output_state {
            OutputState::Outputting(output) => {
                ParsedOutputState::Parsed(ParsedState::parse(output))
            }
            OutputState::Finished => ParsedOutputState::Finished,
            OutputState::Error(error) => ParsedOutputState::Error(ParsedErrorState::parse(error)),
        }
    }
}
