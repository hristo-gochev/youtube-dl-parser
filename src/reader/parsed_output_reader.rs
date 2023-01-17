use crate::reader::output_reader::OutputReader;
use crate::state::parsed_output_state::ParsedOutputState;

/// Parsed output reader
pub struct ParsedOutputReader {
    output_reader: OutputReader,
}

impl ParsedOutputReader {
    /// Creates an output reader that parses the output of a raw reader
    pub fn new(output_reader: OutputReader) -> Self {
        Self { output_reader }
    }
}

impl Iterator for ParsedOutputReader {
    type Item = ParsedOutputState;

    fn next(&mut self) -> Option<Self::Item> {
        self.output_reader.next().map(ParsedOutputState::parse)
    }
}
