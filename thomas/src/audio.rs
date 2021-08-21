use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::{fs::File, io::BufReader};

use std::error::Error;

pub struct Audio {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl Audio {
    /// Basicaly just getting a physical handle to the audio device, we hold onto it
    pub fn new() -> Self {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to load audio device");
        Self {
            _stream,
            stream_handle,
        }
    }
    pub fn play(&mut self, file: File) -> Result<(), Box<dyn Error>> {
        let source = Decoder::new(BufReader::new(file))?;
        self.stream_handle.play_raw(source.convert_samples())?;
        Ok(())
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}
