use crate::helpers::get_asset;
use godot::{
    classes::{AudioStream, AudioStreamPlayer2D, IAudioStreamPlayer2D},
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
            .connect_other(self, |player| {
                if player.restart {
                    player.base_mut().play();
                } else {
                    player.signals().done().emit();
                }
            });
    }
}

impl AudioPlayer {
    pub(crate) fn play(&mut self, path: &str) {
        let stream = get_asset::<AudioStream>(&format!("audio/{path}"));
        self.base_mut().stop();
        self.base_mut().set_stream(&stream);
        self.base_mut().play();
    }
}

#[godot_api]
impl AudioPlayer {
    #[signal]
    pub(crate) fn done();
}
