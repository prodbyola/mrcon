use std::fmt::Display;

pub enum VideoCodec {
    H264,
    H265,
    MPEG4,
}

impl Display for VideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::MPEG4 => "mpeg4",
        };

        write!(f, "{s}")
    }
}

pub enum AudioCodec {
    AAC,
    MP3,
    Vorbis
}

impl Display for AudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AudioCodec::AAC => "aac",
            AudioCodec::MP3 => "mp4",
            AudioCodec::Vorbis => "libvorbis",
        };

        write!(f, "{s}")
    }
}

pub enum OutputFormat {
    MP4,
    MKV,
    MOV
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutputFormat::MP4 => "mp4",
            OutputFormat::MKV => "mkv",
            OutputFormat::MOV => "mov",
        };

        write!(f, "{s}")
    }
}

pub struct VideoScale {
    width: String,
    height: String
}

impl Display for VideoScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

pub enum AudioChannels {
    Stereo, Mono
}

impl Display for AudioChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AudioChannels::Stereo => "stereo",
            AudioChannels::Mono => "mono",
        };

        write!(f, "{s}")
    }
}

pub enum OutputPreset {
    VerySlow,
    Slower,
    Slow,
    Medium,
    Fast,
    Faster,
    VeryFast,
    SuperFast,
    UltraFast
}

impl Display for OutputPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutputPreset::VerySlow => "veryslow",
            OutputPreset::Slower => "slower",
            OutputPreset::Slow => "slow",
            OutputPreset::Medium => "medium",
            OutputPreset::Fast => "fast",
            OutputPreset::Faster => "faster",
            OutputPreset::VeryFast => "veryfast",
            OutputPreset::SuperFast => "superfast",
            OutputPreset::UltraFast => "ultrafast",
        };

        write!(f, "{s}")
    }
}