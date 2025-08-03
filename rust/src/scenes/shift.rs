use std::collections::HashSet;

use crate::{
    helpers::{access, change_scene, get_asset},
    Page,
    player::AudioPlayer, state::{Sanity, State},
};
use godot::{
    classes::{Label, Sprite2D, Texture2D},
    prelude::*,
};
use rand::seq::IndexedRandom;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Shift {
    base: Base<Node>,

    // -- pages --
    sanity: Sanity,
    current: usize,
    seen: HashSet<usize>,

    // -- nodes --
    #[export]
    credits: Option<Gd<Label>>,
    #[export]
    title: Option<Gd<Label>>,
    #[export]
    description: Option<Gd<Label>>,
    #[export]
    page: Option<Gd<Sprite2D>>,

    // -- audio --
    #[export]
    music: Option<Gd<AudioPlayer>>,
    #[export]
    miranda: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl INode for Shift {
    fn ready(&mut self) {
        self.next_page();

        // play the background music
                access(&mut self.music, |player| {
            player.bind_mut().play(&format!("shift/{}", self.sanity));
        });
    }
}

impl Shift {
    fn current_page(&self) -> Page {
        self.sanity.pages()[self.current]
    }

    fn next_page(&mut self) {
        let unseen: Vec<usize> = (0..self.sanity.pages().len())
            .filter(|i| !self.seen.contains(i))
            .collect();

        if unseen.is_empty() {
            let mut state = State::get(&self.base());
            let scene = if self.sanity == Sanity::Hell {
                "give-up"
            } else {
                state.bind_mut().next_shift();
                "fired"
            };
            change_scene(&self.base(), scene);
        } else {
            let &index = unseen
                .choose(&mut rand::rng())
                .expect("should have unseen pages");
            self.seen.insert(index);
            self.current = index;
        }

        self.refresh();
    }

    fn refresh(&mut self) {
        // --- global state ---
        let state = State::get(&self.base());
        self.sanity = state.bind().sanity;

        // credits
        access(&mut self.credits, |label| {
            let credits = state.bind().credits;
            label.set_text(&credits.to_string());
        });

        // -- page --
        let page = self.current_page();

        access(&mut self.title, |label| {
            label.set_text(page.title);
        });
        access(&mut self.description, |label| {
            label.set_text(page.description);
        });

        access(&mut self.page, |sprite| {
            let texture =
                get_asset::<Texture2D>(&format!("pages/{}/{}.webp", self.sanity, self.current));
            sprite.set_texture(&texture);
        });
    }
}

#[godot_api]
impl Shift {
    #[func]
    fn handle_choice(&mut self, accept: bool) {
        // check whether the answer is correct
        let page = self.current_page();
        let correct = page.accept == accept;

        // update the score accordingly
        let mut state = State::get(&self.base());
        state.bind_mut().credits += if correct { page.bonus } else { -page.penalty };

        // praise or scold the player
        access(&mut self.miranda, |player| {
            let max = if self.sanity == Sanity::Hell && !correct { 5 } else { 4 };
            player.bind_mut().play(&format!(
                "{}/{}/{}",
                if correct { "praise" } else { "scold" },
                self.sanity,
                rand::random_range(0..=max)
            ));
        });

        self.next_page();
    }
}
