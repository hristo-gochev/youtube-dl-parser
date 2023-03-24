/*!
Crate `youtube-dl-parser` runs youtube-dl and parses its download output.

# Usage
To use this crate, add `youtube-dl-parser` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
youtube-dl-parser = "0.2.0"
```

# Important
In order to use the functions provided by the library, [youtube-dl](https://github.com/ytdl-org/youtube-dl)
and [FFmpeg](https://github.com/FFmpeg/FFmpeg) must be added to PATH or be in the current directory.

# Work in progress

The crate is currently work in progress and only parses the output of YouTube mp3 and mp4 downloads.

# Example: Start a youtube-dl mp3 download and match on its output:
```
use youtube_dl_parser::expressions::mp3_download_expression;
use youtube_dl_parser::reader::{OutputReader, ParsedOutputReader};
use youtube_dl_parser::state::parsed_state::{
    DeletingState, DownloadState, ErrorState, FFmpegState, ParsedState, YoutubeState,
};
use youtube_dl_parser::state::ParsedOutputState;

let youtube_dl_path = "youtube-dl.exe";
let url = "https://www.youtube.com/watch?v=tPEE9ZwTmy0";
let downloads_folder = "downloads";

let big_cmd = mp3_download_expression(youtube_dl_path, url, downloads_folder);
let stdout = big_cmd
    .stderr_to_stdout()
    .reader()
    .expect("Unable to execute youtube-dl");

let output_reader = OutputReader::new(stdout);

let parsed_output_reader = ParsedOutputReader::new(output_reader);

for parsed_state in parsed_output_reader {
    match parsed_state {
        ParsedOutputState::Parsed(parsed_state) => match parsed_state {
            ParsedState::Youtube(youtube_state) => match youtube_state {
                YoutubeState::Initiating => {
                    println!("Initiating youtube download");
                }
                YoutubeState::ParseError(parse_error) => {
                    println!("YouTube state parse error: {parse_error}");
                }
            },
            ParsedState::Download(download_state) => match download_state {
                DownloadState::Destination(destination) => {
                    println!("Starting download with destination: {destination}");
                }
                DownloadState::Resuming(byte) => {
                    println!("Resuming download from byte: {byte}");
                }
                DownloadState::Downloading(progress, total_size, download_speed, eta) => {
                    println!("Progress: {progress}%, Total size: {total_size} bytes, Download speed: {download_speed} bytes per second, ETA: {eta} seconds");
                }
                DownloadState::Downloaded(progress, total_size, completion_time) => {
                    println!("Download finished at {progress}% of {total_size} bytes in {completion_time} seconds");
                }
                DownloadState::ParseError(err) => {
                    println!("Download state parse error: {err}")
                }
            },
            ParsedState::FFMpeg(ffmpeg_state) => match ffmpeg_state {
                FFmpegState::Destination(destination) => {
                    println!("Using ffmpeg with file destination: {destination}");
                }
                FFmpegState::ParseError(parse_error) => {
                    println!("FFmpeg state parse error: {parse_error}");
                }
            },
            ParsedState::Error(error_state) => match error_state {
                ErrorState::Error(error) => {
                    println!("Error occurred: {error}")
                }
            },
            ParsedState::Deleting(deleting_state) => match deleting_state {
                DeletingState::DeletingTemporaryFile(path) => {
                    println!("Deleting temporary file: {path}");
                }
                DeletingState::ParseError(parse_error) => {
                    println!("Deleting state parse error: {parse_error}");
                }
            },
            ParsedState::None(output) => {
                println!("No state detected: {output}")
            }
            ParsedState::Unknown(output) => {
                println!("Unknown output state: {output}");
            }
            ParsedState::ParseError(parse_error) => {
                println!("Output parse error: {parse_error}");
            }
        },
        ParsedOutputState::Finished => {
            println!("Output finished");
        }
        ParsedOutputState::Error(parsed_error_state) => {
            println!(
                "Exit code: {:?}, Error: {}",
                parsed_error_state.exit_code, parsed_error_state.error
            );
        }
    }
}
```

 */

/// Duct expressions for mp3 and mp4 youtube-dl downloads.
pub mod expressions;
/// Parsed and raw output readers
pub mod reader;
/// The different states the readers could output
pub mod state;
