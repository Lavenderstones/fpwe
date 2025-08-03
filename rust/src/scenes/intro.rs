use crate::{
    helpers::{access, change_scene},
    player::AudioPlayer,
    state::{Sanity, State},
};
use godot::{
    classes::{AnimatedSprite2D, InputEvent, InputEventKey},
    global::Key,
    prelude::*,
};

const PLAYER_OPTIONS: [u8; 3] = [0, 1, 1];
const LAST_SNIPPET: u8 = 4;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Intro {
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

        // choose a random name
        let state = State::get(&self.base());
        let sanity = state.bind().sanity;
        let max = PLAYER_OPTIONS[sanity as usize];
        let name = rand::random_range(0..=max);

        // start audio
        access(&mut self.audio, |player| {
            play(
                &mut player.bind_mut(),
                0,
                format!("employee/{sanity}/{name}"),
                format!("company/{sanity}"),
            );
        });
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let state = State::get(&self.base());
        if let Ok(key) = event.try_cast::<InputEventKey>()
            && key.is_pressed()
            && (state.bind().sanity != Sanity::Normal || cfg!(debug_assertions))
        {
            match key.get_keycode() {
                Key::SPACE | Key::ENTER => {
                    // skip the intro
                    change_scene(&self.base(), "shift");
                }
                _ => {}
            }
        }
    }
}

fn play(player: &mut AudioPlayer, intro: u8, name: String, company: String) {
    if intro > LAST_SNIPPET {
        return change_scene(&player.base(), "shift");
    }

    // play the intro audio
    player.play(&format!("intro/{}", intro));

    // at the end of the audio, play the next custom audio
    if intro == LAST_SNIPPET {
        return change_scene(&player.base(), "shift");
    }

    player.signals().done().connect_self(move |player| {
        match intro {
            0 => player.play(&name),
            _ => player.play(&company),
        }

        // at the end of the audio, play the next snippet
        {
            let name = name.clone();
            let company = company.clone();

            player.signals().done().connect_self(move |player| {
                play(player, intro + 1, name.clone(), company.clone());
            });
        }
    });
}
