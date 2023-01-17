# youtube-dl-parser

Runs youtube-dl and parses its download output.

### Usage

To use this crate, add `youtube-dl-parser` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
youtube-dl-parser = "0.1.0"
```

### Important

In order to use the functions provided by the library, [youtube-dl](https://github.com/ytdl-org/youtube-dl)
and [FFmpeg](https://github.com/FFmpeg/FFmpeg) must be added to PATH or be in the current directory.

### Work in progress

The crate is currently work in progress and only parses the output of YouTube mp3 and mp4 downloads.

### Features

* Provides an output reader for youtube-dl processes that parses each line.
* Provides a raw output reader for youtube-dl process.
* Exposes duct expressions for mp3 and mp4 youtube-dl downloads.

### Contributing

Contributions to extend this crate are greatly appreciated!