extern crate ffmpeg_next as ffmpeg;

mod cli;
mod video;

use anyhow::Result;

fn main() -> Result<()> {
    ffmpeg::init()?;
    cli::run()
}
