use rodio::{Decoder, OutputStream, OutputStreamHandle, Sample, Sink, Source};
use std::{fs::File, io::BufReader};

use std::error::Error;

use crate::context::Context;

/// Aims to be as light of a wrapper over rodio as possible, since the default rodio API is already pretty good IMO
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
}

/// Mainly useful for playing sound effects I think?
pub fn play_once_vorbis(ctx: &mut Context, path: &str) -> Result<(), Box<dyn Error>> {
    // Load the file via resource manager
    let file = ctx.resource_mgr.load_file(path)?;
    let sink = rodio::Sink::try_new(&ctx.audio.stream_handle)?;
    sink.append(rodio::Decoder::new_vorbis(BufReader::new(file))?);
    sink.detach();
    Ok(())
}

/// Makes it so that the audio can be configured
pub fn configure_audio_vorbis(
    ctx: &mut Context,
    path: &str,
) -> Result<Decoder<BufReader<File>>, Box<dyn Error>> {
    let file = ctx.resource_mgr.load_file(path)?;
    Ok(rodio::Decoder::new_vorbis(BufReader::new(file))?)
}

/// Useful for configuration, returns the sink
pub fn play_source<T>(ctx: &mut Context, src: T) -> Result<Sink, Box<dyn Error>>
where
    T: Source + Send + 'static,
    T::Item: Sample + Send,
{
    let sink = rodio::Sink::try_new(&ctx.audio.stream_handle)?;
    sink.append(src);
    Ok(sink)
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}
