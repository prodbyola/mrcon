use params::{AudioChannels, AudioCodec, OutputFormat, OutputPreset, VideoScale};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use self::params::VideoCodec;
use crate::{probe::Probe, utility::string};

pub(self) mod params;

#[derive(Default)]
pub struct ConversionOption {
    pub input: String,
    pub output: String,
    pub audio_bitrate: Option<usize>,
    pub audio_code: Option<AudioCodec>,
    pub video_code: Option<VideoCodec>,
    pub video_bitrate: Option<usize>,
    pub output_format: Option<OutputFormat>,
    pub resolution: Option<VideoScale>,
    pub audio_sample_rate: Option<usize>,
    pub channels: Option<AudioChannels>,
    pub video_quality: Option<u8>,
    pub preset: Option<OutputPreset>,
    pub overwrite: bool,
}

impl<'a> IntoIterator for &'a ConversionOption {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        let mut opts = vec![
            string("-i"),
            string(&self.input),
            string("-progress"),
            string("pipe:1"),
        ];

        // video codec
        if let Some(vc) = &self.video_code {
            opts.push(format!("-c:v {vc}"));
        }

        // set audio codec
        if let Some(ac) = &self.audio_code {
            opts.push(format!("-c:a {ac}"));
        }

        // output format
        if let Some(f) = &self.output_format {
            opts.push(format!("-f {f}"));
        }

        // video bitrate
        if let Some(br) = self.video_bitrate {
            opts.push(format!("-b:v {br}"));
        }

        // audio bitrate
        if let Some(br) = self.audio_bitrate {
            opts.push(format!("-a:v {br}"));
        }

        // video scale
        if let Some(scale) = &self.resolution {
            opts.push(format!("-s {scale}"));
        }

        // audio sample rate
        if let Some(rate) = self.audio_sample_rate {
            opts.push(format!("{rate}"));
        }

        // channels 
        if let Some(ch) = &self.channels {
            let ch: u8 = match ch {
                AudioChannels::Mono => 1,
                AudioChannels::Stereo => 2
            };

            opts.push(format!("-ac {ch}"));
        }

        // video quality
        if let Some(vq) = self.video_quality {
            opts.push(format!("-crf {vq}"));
        }

        // preset
        if let Some(preset) = &self.preset {
            opts.push(format!("-preset {preset}"));
        }

        // output
        opts.push(self.output.to_string());
        if self.overwrite {
            opts.push("-y".to_string());
        }

        opts.into_iter()
    }
}

/// Run conversion of an input file to an output file
pub async fn run(opts: ConversionOption) -> std::io::Result<()> {
    let mut cmd = Command::new("ffmpeg")
        .args(opts.into_iter())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let out = cmd.stdout.take().unwrap();
    let err = cmd.stderr.take().unwrap();

    let out_reader = BufReader::new(out);
    let err_reader = BufReader::new(err);

    let mut out_lines = out_reader.lines();
    let mut err_lines = err_reader.lines();

    let probe = Probe::new(&opts.input)?;

    tokio::spawn(async move {
        while let Some(line) = out_lines.next_line().await.unwrap() {
            if line.starts_with("out_time_us") {
                let split: Vec<&str> = line.split("=").collect();

                let ds = *(split.get(1).unwrap_or(&"0")); // duration str
                let pd = ds.parse::<f64>().unwrap_or_default(); // parse duration

                let current_duration = pd / 1e6;
                let total_duration = probe.duration;
                let percent = ((current_duration * 100.0) / total_duration).round();
                println!(
                    "total: {total_duration}, current: {current_duration}, percent: {percent}"
                );
            }
        }
    })
    .await?;

    tokio::spawn(async move {
        while let Some(line) = err_lines.next_line().await.unwrap() {
            eprintln!("stderr: {}", line);
        }
    })
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::converter::{self, ConversionOption};

    #[tokio::test]
    async fn test_converter() {
        let input = "demo.mp4".to_string();
        let output = "output.mp4".to_string();

        let opts = ConversionOption {
            input,
            output,
            ..Default::default()
        };

        let con = converter::run(opts).await;
        assert!(con.is_ok())
    }
}
