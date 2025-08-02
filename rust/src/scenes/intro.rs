use crate::{
    helpers::{access, get_path},
    player::AudioPlayer,
};
use godot::{classes::AnimatedSprite2D, prelude::*};

const LAST_SNIPPET: u8 = 4;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Intro {
    base: Base<Node>,
    #[export]
    background: Option<Gd<AnimatedSprite2D>>,
    #[export]
    audio: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl INode for Intro {
    fn ready(&mut self) {
        // play the background animation
        access(&mut self.background, |sprite| {
            sprite.play();
        });

        access(&mut self.audio, |player| {
            Intro::play_snippet(&mut player.bind_mut(), 0);
        });
    }
}

impl Intro {
    fn play_snippet(player: &mut AudioPlayer, snippet: u8) {
        if snippet > LAST_SNIPPET
            && let Some(mut tree) = player.base().get_tree()
        {
            tree.change_scene_to_file(&get_path("scenes/shift.tscn"));
        }

        // play the intro audio
        player.play(&format!("miranda/intro/{}.ogg", snippet));

        // at the end of the audio, play the next snippet
        let conn = player.signals().done().connect_self(move |player| {
            Intro::play_snippet(player, snippet + 1);
        });
        conn.disconnect();
    }
}
