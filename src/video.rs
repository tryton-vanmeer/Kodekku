use std::{fmt, path::PathBuf};

use anyhow::{Context, Result};
use colored::Colorize;

#[derive(Debug)]
pub struct Video {
    filename: String,
    codec: String,
}

impl Video {
    pub fn new(path: &str) -> Result<Self> {
        let filename = PathBuf::from(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let input = ffmpeg::format::input(&path)?;
        let stream = input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .context("no video stream found or not a video file")?;
        let context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
        let codec = context
            .decoder()
            .video()?
            .codec()
            .unwrap()
            .id()
            .name()
            .to_string();

        Ok(Video { filename, codec })
    }
}

impl fmt::Display for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.codec.green().bold(),
            self.filename.purple().bold()
        )
    }
}
