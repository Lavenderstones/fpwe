use godot::{
    classes::{AudioStream, AudioStreamPlayer2D, IAudioStreamPlayer2D, ResourceLoader},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = AudioStreamPlayer2D)]
pub(crate) struct AudioPlayer {
    base: Base<AudioStreamPlayer2D>,

    /// Should the audio restart when finished?
    #[export]
    restart: bool,
}

#[godot_api]
impl IAudioStreamPlayer2D for AudioPlayer {
    fn ready(&mut self) {
        self.base()
            .signals()
            .finished()
            .connect_other(self, Self::on_finish);
    }
}

impl AudioPlayer {
    /// Run when the current [AudioStream] finishes playing.
    fn on_finish(&mut self) {
        if self.restart {
            self.base_mut().play();
        }
    }

    /// Load an [AudioStream] from the given path.
    pub(crate) fn load(path: &str) -> Gd<AudioStream> {
        ResourceLoader::singleton()
            .load(&format!("res://assets/{}", path))
            .and_then(|res| res.try_cast::<AudioStream>().ok())
            .expect("audio must be valid")
    }

    /// Play the [AudioStream] from the given path on the [AudioPlayer].
    pub(crate) fn play(&mut self, path: &str) {
        self.base_mut().stop();
        self.base_mut().set_stream(&Self::load(path));
        self.base_mut().play();
    }
}
