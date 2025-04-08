use std::{
    io::{Error, ErrorKind},
    process::Command,
};

use serde_json::{Map, Value};

#[derive(Debug)]
/// A wrapper for [FFProbe](https://www.ffmpeg.org/ffprobe.html), which is a libray that
/// gathers information from multimedia streams and prints it in human- and machine-readable fashion.
pub struct Probe {
    pub filename: String,
    pub format: String,
    pub size: usize,
    pub duration: f64,
    pub bitrate: usize,
    pub title: String,
}

impl Probe {
    /// Create a new instance of [Probe] for an input file(name).
    pub fn new(input: &str) -> std::io::Result<Self> {
        let cmd = Command::new("ffprobe")
            .args([
                "-show_entries",
                "stream_tags : format_tags",
                "-show_format",
                "-print_format",
                "json",
                "-loglevel",
                "quiet",
                input,
            ])
            .output()?;

        let data = Self::parse_data(cmd.stdout)?;
        let format = data["format"].as_object().unwrap();

        let filename = format["filename"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let format_name = format["format_name"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let size = format["size"]
            .as_str()
            .map(|s| s.parse::<usize>().unwrap_or_default())
            .unwrap_or_default();
        let duration = format["duration"]
            .as_str()
            .map(|s| s.parse::<f64>().unwrap_or_default())
            .unwrap_or_default();
        let bitrate = format["bit_rate"]
            .as_str()
            .map(|s| s.parse::<usize>().unwrap_or_default())
            .unwrap_or_default();

        let tags = format["tags"].as_object().unwrap();
        let title = tags
            .get("title")
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        let probe = Self {
            filename,
            format: format_name,
            size,
            duration,
            bitrate,
            title,
        };

        Ok(probe)
    }

    /// Parses raw data into JSON Map type.
    fn parse_data(data: Vec<u8>) -> std::io::Result<Map<String, Value>> {
        match String::from_utf8(data) {
            Ok(data) => {
                let parsed: Value = serde_json::from_str(&data).map_err(|_| {
                    Error::new(
                        ErrorKind::Interrupted,
                        "fail to convert probe data to value",
                    )
                })?;

                match parsed.as_object() {
                    Some(output) => Ok(output.clone()),
                    None => Err(Error::new(
                        ErrorKind::NotFound,
                        "error creating object from data",
                    )),
                }
            }
            Err(e) => Err(Error::new(ErrorKind::UnexpectedEof, e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::probe::Probe;

    #[test]
    fn test_probe() {
        let pb = Probe::new("demo.mkv");
        assert!(pb.is_ok());
    }
}
