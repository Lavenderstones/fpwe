use godot::{
    classes::{AudioStreamPlayer2D, IAudioStreamPlayer2D},
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
}
