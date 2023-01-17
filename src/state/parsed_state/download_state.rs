use anyhow::{anyhow, bail, Error, Result};

/// Occurs when a download is in progress
pub enum DownloadState {
    Destination(String),
    Resuming(u64),
    Downloading(f32, u64, u64, u64),
    Downloaded(f32, u64, u64),
    ParseError(Error),
}

impl DownloadState {
    pub fn parse<'a>(mut split: impl DoubleEndedIterator<Item = &'a str> + Send) -> DownloadState {
        let Some(download_indicator) = split.next() else { return DownloadState::ParseError(anyhow!("No download indicator detected")); };
        match download_indicator {
            "Resuming" => match Self::parse_resuming(split) {
                Ok(state) => state,
                Err(err) => DownloadState::ParseError(err),
            },
            "Destination:" => Self::parse_destination(split),
            _ => match Self::parse_progress(download_indicator, split) {
                Ok(state) => state,
                Err(err) => DownloadState::ParseError(err),
            },
        }
    }

    fn parse_resuming<'a>(
        mut split: impl DoubleEndedIterator<Item = &'a str> + Send,
    ) -> Result<DownloadState> {
        split.next();
        split.next();
        split.next();
        let Some(byte) = split.next() else { bail!("No resume byte detected") };
        let byte = Self::parse_resume_byte(byte)?;
        Ok(DownloadState::Resuming(byte))
    }

    fn parse_destination<'a>(
        split: impl DoubleEndedIterator<Item = &'a str> + Send,
    ) -> DownloadState {
        let destination = split.collect::<Vec<&str>>().join(" ");
        DownloadState::Destination(destination)
    }

    fn parse_progress<'a>(
        download_indicator: &str,
        mut split: impl DoubleEndedIterator<Item = &'a str> + Send,
    ) -> Result<DownloadState> {
        // Parse progress
        let progress = Self::parse_percentage(download_indicator).map_err(|err| {
            anyhow!("Unable to parse progress \'{download_indicator}\' with error: {err}")
        })?;

        // Skip "of" text
        split.next();

        // Parse total size
        let Some(total_size) = split.next() else { bail!("No total size detected"); };
        let total_size = Self::parse_total_size(total_size).map_err(|err| {
            anyhow!("Unable to parse total size \'{total_size}\' with error: {err}")
        })?;

        // Parse if still downloading
        let Some(in_or_at)=  split.next() else {bail!("Unable to get if still downloading")};

        match in_or_at.trim() {
            "at" => {
                // Parse download speed
                let Some(download_speed) = split.next() else { bail!("No download speed detected"); };
                let download_speed = Self::parse_download_speed(download_speed).map_err(|err| {
                    anyhow!("Unable to parse download speed \'{download_speed}\' with error: {err}")
                })?;
                // Skip "ETA" text
                split.next();
                // Parse ETA
                let Some(eta) = split.next() else { bail!("No ETA detected"); };
                let eta = Self::parse_time(eta)
                    .map_err(|err| anyhow!("Unable to parse eta \'{eta}\' with error: {err}"))?;

                Ok(DownloadState::Downloading(
                    progress,
                    total_size,
                    download_speed,
                    eta,
                ))
            }
            "in" => {
                // Parse completion time
                let Some(completion_time) = split.next() else { bail!("No completion time detected"); };
                let completion_time = Self::parse_time(completion_time).map_err(|err| {
                    anyhow!("Unable to parse completion \'{completion_time}\' with error: {err}")
                })?;

                Ok(DownloadState::Downloaded(
                    progress,
                    total_size,
                    completion_time,
                ))
            }
            _ => {
                bail!("Unable to get if still downloading");
            }
        }
    }

    fn parse_resume_byte(byte: &str) -> Result<u64> {
        byte.trim().parse::<u64>().map_err(anyhow::Error::from)
    }

    fn parse_percentage(percentage: &str) -> Result<f32> {
        let mut chars = percentage.chars();
        chars.next_back();
        chars
            .as_str()
            .trim()
            .parse::<f32>()
            .map_err(anyhow::Error::from)
    }

    fn parse_total_size(size_str: &str) -> Result<u64> {
        // Split the string into a value and unit

        let Some(last_digit_index) = size_str.rfind(|char: char| char.is_ascii_digit()) else{
            bail!("Incorrectly formatted size string");
        };
        let (value_str, unit) = size_str.split_at(last_digit_index + 1);

        // Parse the value as a float
        let value: f64 = value_str.trim().parse().map_err(anyhow::Error::from)?;

        // Convert the value to bytes based on the unit
        let bytes = match unit {
            "B" => value,
            "KB" => value * 1_000.0,
            "MB" => value * 1_000_000.0,
            "GB" => value * 1_000_000_000.0,
            "TB" => value * 1_000_000_000_000.0,
            "KiB" => value * 1_024.0,
            "MiB" => value * 1_048_576.0,
            "GiB" => value * 1_073_741_824.0,
            "TiB" => value * 1_099_511_627_776.0,
            _ => bail!("Unrecognized unit: {}", unit),
        } as u64;

        Ok(bytes)
    }

    fn parse_download_speed(size_str: &str) -> Result<u64> {
        // Split the string into a value and unit

        let Some(last_digit_index) = size_str.rfind(|char: char| char.is_ascii_digit()) else{
            bail!("Incorrectly formatted size");
        };

        let (value_str, mut unit) = size_str.split_at(last_digit_index + 1);

        // Parse the value as a float
        let value: f64 = value_str.trim().parse().map_err(anyhow::Error::from)?;

        unit = &unit[..unit.len() - 2];

        // Convert the value to bytes based on the unit
        let bytes = match unit {
            "B" => value,
            "KB" => value * 1_000.0,
            "MB" => value * 1_000_000.0,
            "GB" => value * 1_000_000_000.0,
            "TB" => value * 1_000_000_000_000.0,
            "KiB" => value * 1_024.0,
            "MiB" => value * 1_048_576.0,
            "GiB" => value * 1_073_741_824.0,
            "TiB" => value * 1_099_511_627_776.0,
            _ => bail!("Unrecognized unit: {}", unit),
        } as u64;

        Ok(bytes)
    }

    fn parse_time(time: &str) -> Result<u64> {
        let parts: Vec<&str> = time.split(':').collect();
        match parts.len() {
            1 => {
                // Time is in the format "SS"
                let seconds = parts[0].parse::<u64>()?;
                Ok(seconds)
            }
            2 => {
                // Time is in the format "MM:SS"
                let minutes = parts[0].parse::<u64>()?;
                let seconds = parts[1].parse::<u64>()?;
                Ok((minutes * 60) + seconds)
            }
            3 => {
                // Time is in the format "HH:MM:SS"
                let hours = parts[0].parse::<u64>()?;
                let minutes = parts[1].parse::<u64>()?;
                let seconds = parts[2].parse::<u64>()?;
                Ok((hours * 3600) + (minutes * 60) + seconds)
            }
            4 => {
                // Time is in the format "DD:HH:MM:SS"
                let days = parts[0].parse::<u64>()?;
                let hours = parts[1].parse::<u64>()?;
                let minutes = parts[2].parse::<u64>()?;
                let seconds = parts[3].parse::<u64>()?;
                Ok((days * 86400) + (hours * 3600) + (minutes * 60) + seconds)
            }
            _ => {
                // Time is in an invalid format
                Err(anyhow!("Invalid time format"))
            }
        }
    }
}
