extern crate ffmpeg_next as ffmpeg;

#[derive(Debug)]
pub struct Video {
    filename: String,
    codec: String,
}

impl Video {
    pub fn new(path: &str) -> Self {
        ffmpeg::init().unwrap();

        Video {
            filename: path.to_string(),
            codec: "test".to_string(),
        }
    }
}
