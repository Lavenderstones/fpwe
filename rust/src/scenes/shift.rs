use godot::{classes::Label, prelude::*};

use crate::{helpers::get_state, player::AudioPlayer};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Shift {
    base: Base<Node>,

    // -- labels --
    #[export]
    credits: Option<Gd<Label>>,

    // -- audio --
    #[export]
    music: Option<Gd<AudioPlayer>>,
    #[export]
    miranda: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl INode for Shift {
    fn ready(&mut self) {
        self.update();
    }
}

impl Shift {
    /// Update labels in the scene.
    fn update(&mut self) {
        // --- global state ---
        let mut state = get_state(&self.base());

        // music
        self.music.as_mut().map(|player| {
            let sanity = state.call("get_sanity", &[]).to::<i32>();
            player.bind_mut().play(&format!("shift/{sanity}.ogg"));
        });

        // credits
        self.credits.as_mut().map(|label| {
            let credits = state.call("get_credits", &[]).to::<i32>();
            label.set_text(&credits.to_string());
        });
    }
}
