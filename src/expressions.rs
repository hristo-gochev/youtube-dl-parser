use duct::{cmd, Expression};

/// Creates a mp3 download expression based on the given params
pub fn mp3_download_expression(
    youtube_dl_path: &str,
    url: &str,
    download_folder: &str,
) -> Expression {
    let download_path = format!("{download_folder}\\%(title)s.%(ext)s");

    cmd!(
        youtube_dl_path,
        "-x",
        "--audio-format",
        "mp3",
        "-o",
        download_path,
        url
    )
}

/// Formats a mp4 download expression based on the given params
pub fn mp4_download_expression(
    youtube_dl_path: &str,
    url: &str,
    download_folder: &str,
) -> Expression {
    let download_path = format!("{download_folder}\\%(title)s.%(ext)s");

    cmd!(
        youtube_dl_path,
        "--merge-output-format",
        "mp4",
        "-o",
        download_path,
        url
    )
}
