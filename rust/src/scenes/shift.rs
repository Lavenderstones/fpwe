use std::collections::HashSet;

use crate::{
    helpers::{access, get_asset, get_state},
    page::Page,
    player::AudioPlayer,
};
use godot::{
    classes::{Label, Sprite2D, Texture2D},
    prelude::*,
};

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
        // select a page
        let pages = Page::for_sanity(self.sanity);
        self.current = rand::random_range(0..pages.len());
        self.seen.insert(self.current);
        // refresh the screen
        self.refresh();
    }
}

impl Shift {
    fn current_page(&self) -> Page {
        let pages = Page::for_sanity(self.sanity);
        pages[self.current].clone()
    }

    fn refresh(&mut self) {
        // --- global state ---
        let mut state = get_state(&self.base());
        self.sanity = state.call("get_sanity", &[]).to::<u8>();

        // music
        access(&mut self.music, |player| {
            player
                .bind_mut()
                .play(&format!("shift/{}.ogg", self.sanity));
        });

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
