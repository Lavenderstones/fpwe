use crate::{
    helpers::{access, change_scene},
    player::AudioPlayer,
    state::State,
};
use godot::{
    classes::{AnimatedSprite2D, Label},
    prelude::*,
};

const FIRED_SOUND_COUNT: u8 = 5;

const FIRE_MESSAGES: [&'static str; 13] = [
    "You were fired for bad odour.",
    "You were fired for sitting in a funny position.",
    "You're fired. We know you did it.\nWhat's 'it'? No thanks, I've just had lunch.",
    "You were fired for your Louis Armstrong impression. It's getting old.",
    "You were struck by the banhammer! I, your manager, am become meme.",
    "Fired.\nI asked for a sandwich\nYou said 'You want beef?'\nGet out my office.\nWasteman.",
    "We've decided to move on with another applicant",
    "You are done. Fired. Do not show your face at the office again.",
    "This is your shot at freedom, start running and dont turn ba-\nSorry, we had to fire who wrote these messages for treason.",
    "You can run but you cant hide. y̵̨̩̱̓ö̷́̈͠͝u̷͚̓̊͘ ̶̯̒̔͜c̵̦͕̘̄a̵̟̯̅̓n̵̫͂̓̔ ̸̆͒͊͝n̶̆́̑͝e̴̛͌̅ver h̴̑̽̂͋ỉ̴̒̚͝ḑ̶̃ę̸͉͒͛",
    "Fired for rebellious home lifestyle.",
    "Fired for contraband. Didn't we say tell you to keep away from books?",
    "Your execution has been scheduled for 3 hours. Thank you for working at our company, valued employee!",
];

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Fired {
    base: Base<Node>,

    #[export]
    background: Option<Gd<AnimatedSprite2D>>,
    #[export]
    player: Option<Gd<AudioPlayer>>,
    #[export]
    fired: Option<Gd<Label>>,
}

#[godot_api]
impl INode for Fired {
    fn ready(&mut self) {
        // play the background animation
        access(&mut self.background, |sprite| {
            sprite.play();
        });

        access(&mut self.fired, |label| {
            let i = rand::random_range(0..FIRE_MESSAGES.len());
            label.set_text(FIRE_MESSAGES[i]);
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
