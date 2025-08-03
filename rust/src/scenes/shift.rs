use std::collections::HashSet;

use crate::{
    helpers::{access, change_scene, get_asset, get_state},
    page::Page,
    player::AudioPlayer,
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
    sanity: u8,
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
        // music
        access(&mut self.music, |player| {
            player.bind_mut().play(&format!("shift/{}", self.sanity));
        });

        self.next_page();
    }
}

impl Shift {
    fn current_page(&self) -> Page {
        let pages = Page::for_sanity(self.sanity);
        pages[self.current].clone()
    }

    fn next_page(&mut self) {
        let pages = Page::for_sanity(self.sanity);
        let unseen: Vec<usize> = (0..pages.len())
            .filter(|i| !self.seen.contains(i))
            .collect();

        if unseen.is_empty() {
            let mut state = get_state(&self.base());

            let scene = if self.sanity == 2 {
                "give-up"
            } else {
                state.call("next_sanity", &[]);
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
        let mut state = get_state(&self.base());
        self.sanity = state.call("get_sanity", &[]).to::<u8>();

        // credits
        access(&mut self.credits, |label| {
            let credits = state.call("get_credits", &[]).to::<i32>();
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
        let credit_delta = if correct { page.bonus } else { -page.penalty };

        // update the score accordingly
        let mut state = get_state(&self.base());
        state.call("update_credits", &[Variant::from(credit_delta)]);

        // praise or scold the player
        access(&mut self.miranda, |player| {
            let max = if self.sanity == 2 && !correct { 5 } else { 4 };
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
