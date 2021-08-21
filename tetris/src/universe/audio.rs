use thomas::{context::Context, rodio::Sink};

/// TODO Problem: Currently we initalize universe before context
/// In order to initialize audio, we must then have access to context
/// Therefore we must separate config laoding from game loading huh
pub struct Audio {
    // Plays in the background for the enter game
    pub sink: Option<Sink>,
}

impl Audio {
    /// Currently context isn't used but it probably will in the future
    pub fn new(_ctx: &mut Context) -> Self {
        Self { sink: None }
    }
}
