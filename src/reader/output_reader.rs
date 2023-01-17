use anyhow::anyhow;
use duct::ReaderHandle;
use std::collections::VecDeque;
use std::io::{BufReader, Read};

use crate::state::output_state::OutputState;

/// Raw output reader
pub struct OutputReader {
    stdout_reader: BufReader<ReaderHandle>,
    failed: bool,
    finished: bool,
    buffer_capacity: usize,
    line_queue: VecDeque<String>,
}

impl OutputReader {
    /// Creates a raw output reader based on the given reader handle
    pub fn new(child_stdout: ReaderHandle) -> Self {
        let stdout_reader = BufReader::new(child_stdout);
        let buffer_capacity = stdout_reader.capacity();
        let line_queue = VecDeque::new();
        Self {
            stdout_reader,
            failed: false,
            finished: false,
            buffer_capacity,
            line_queue,
        }
    }
}

impl Iterator for OutputReader {
    type Item = OutputState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed || self.finished {
            return None;
        }

        if !self.line_queue.is_empty() {
            let Some(next_output) = self.line_queue.pop_front() else {
                self.failed=true;
                return Some(OutputState::Error(anyhow!("Failed to read an output")));
            };
            return Some(OutputState::Outputting(next_output));
        }

        let mut output = vec![0; self.buffer_capacity];

        let read_size = match self.stdout_reader.read(&mut output) {
            Ok(read_size) => read_size,
            Err(err) => {
                self.failed = true;
                return Some(OutputState::Error(err.into()));
            }
        };

        if read_size == 0 {
            self.finished = true;
            return Some(OutputState::Finished);
        }

        output.truncate(read_size);

        let mut output = String::from_utf8_lossy(output.as_slice()).to_string();

        output = output.replace(['\n'], "");

        if !output.contains('\x0D') {
            return Some(OutputState::Outputting(output));
        }

        let mut output = output
            .split('\x0D')
            .skip(1)
            .map(|str| str.to_owned())
            .collect::<VecDeque<_>>();

        let Some(that_one) = output.pop_front() else {
            self.failed=true;
            return Some(OutputState::Error(anyhow!("Failed to read an output")));
        };

        if output.is_empty() {
            Some(OutputState::Outputting(that_one))
        } else {
            self.line_queue = output;
            Some(OutputState::Outputting(that_one))
        }
    }
}
