use crate::{
    helpers::{access, get_path, get_state},
    player::AudioPlayer,
};
use godot::{classes::AnimatedSprite2D, prelude::*};
#[cfg(debug_assertions)]
use godot::{
    classes::{InputEvent, InputEventKey},
    global::Key,
};

const PLAYER_OPTIONS: [u8; 3] = [0, 1, 1];
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

        // choose a random name
        let mut state = get_state(&self.base());
        let sanity = state.call("get_sanity", &[]).to::<u8>();
        let max = PLAYER_OPTIONS[usize::from(sanity)];
        let name = rand::random_range(0..=max);

        // get the company
        let company = state.call("get_company", &[]).to::<u8>();

        // start audio
        access(&mut self.audio, |player| {
            play(
                &mut player.bind_mut(),
                0,
                format!("miranda/employee/{sanity}/{name}"),
                format!("miranda/company/{company}"),
            );
        });
    }

    #[cfg(debug_assertions)]
    fn input(&mut self, event: Gd<InputEvent>) {
        // skip the intro in dev builds
        if let Ok(key) = event.try_cast::<InputEventKey>()
            && key.is_pressed()
        {
            match key.get_keycode() {
                Key::SPACE | Key::ENTER => {
                    // skip the intro
                    if let Some(mut tree) = self.base().get_tree() {
                        tree.change_scene_to_file(&get_path("scenes/shift.tscn"));
                    }
                }
                _ => {}
            }
        }
    }
}

fn play(player: &mut AudioPlayer, intro: u8, name: String, company: String) {
    if intro > LAST_SNIPPET
        && let Some(mut tree) = player.base().get_tree()
    {
        tree.change_scene_to_file(&get_path("scenes/shift.tscn"));
    }

    // play the intro audio
    player.play(&format!("miranda/intro/{}", intro));

    // at the end of the audio, play the next custom audio
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
