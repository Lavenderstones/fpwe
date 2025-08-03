use crate::{
    helpers::{access, change_scene},
    player::AudioPlayer,
    state::State,
};
use godot::{classes::AnimatedSprite2D, prelude::*};

const FIRED_SOUND_COUNT: u8 = 5;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Fired {
    base: Base<Node>,

    #[export]
    background: Option<Gd<AnimatedSprite2D>>,
    #[export]
    player: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl INode for Fired {
    fn ready(&mut self) {
        // play the background animation
        access(&mut self.background, |sprite| {
            sprite.play();
        });

        // play a random fired audio
        let mut state = State::get(&self.base());
        access(&mut self.player, |player| {
            let i = rand::random_range(0..FIRED_SOUND_COUNT);
            state.bind_mut().fired_seen.insert(i);
            player.bind_mut().play(&format!("fired/{i}"));

            player.signals().done().connect_self(|player| {
                change_scene(&player.base(), "intro");
            });
        });
    }
}
