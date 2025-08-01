use crate::{assets, player::AudioPlayer, shift::{data, Shift}};
use godot::{
    classes::{Label, ResourceLoader, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct Game {
    base: Base<Node>,
    shift: Shift,
    shift_id: usize,

    /// The current score of the player.
    score: isize,
    /// The label to display the score.
    #[export]
    score_label: Option<Gd<Label>>,

    /// The sprite for the current page.
    #[export]
    page: Option<Gd<Sprite2D>>,
    /// The title of the current page.
    #[export]
    title: Option<Gd<Label>>,
    /// The description of the current page.
    #[export]
    description: Option<Gd<Label>>,

    /// The music player for the game.
    #[export]
    music: Option<Gd<AudioPlayer>>,
    /// Miranda's audio player.
    #[export]
    miranda: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        self.update(true);
    }
}

impl Game {
    /// Update fields in [Game] that depend on the current [Shift].
    fn update(&mut self, music: bool) {
        // set the music
        if music && let Some(player) = self.music.as_mut() {
            player
                .bind_mut()
                .play(&format!("shift/{}.ogg", self.shift_id));
        }

        // set the page texture
        self.page.as_mut().map(|sprite| {
            ResourceLoader::singleton()
                .load(&assets(&format!(
                    "pages/{}/{}.webp",
                    self.shift_id, self.shift.index
                )))
                .and_then(|res| res.try_cast::<Texture2D>().ok())
                .map(|texture| sprite.set_texture(&texture))
        });

        // set the page title
        let page = self.shift.page();
        if let Some(label) = self.title.as_mut() {
            label.set_text(page.title);
        }

        // set the page description
        if let Some(label) = self.description.as_mut() {
            label.set_text(page.description);
        }
    }

    fn next_page(&mut self) {
        let music = if self.shift.is_done() {
            self.shift_id += 1;
            match self.shift_id {
                1 => self.shift = (&data::TIRED[..]).into(),
                2 => self.shift = (&data::HORRORS[..]).into(),
                _ => {
                    // todo: end game
                    self.shift_id = 0;
                    self.shift = (&data::REGULAR[..]).into();
                },
            }
            true
        } else {
            false
        };
        self.shift.next();
        self.update(music);
    }

}

#[godot_api]
impl Game {
    #[func]
    fn handle_choice(&mut self, answer: bool) {
        let page = self.shift.page();

        // update score
        let (praise, dx) = page.check(answer);
        self.score += dx;
        if let Some(label) = self.score_label.as_mut() {
            label.set_text(&self.score.to_string());
        }

        // praise/scold
        if let Some(player) = self.miranda.as_mut() {
            let category = if praise { "praise" } else { "scold" };
            let end = if !praise && self.shift_id == 2 { 5 } else { 4};
            let i = rand::random_range(0..=end);
            
            player.bind_mut().play(&format!(
                "miranda/{}/{}/{}.ogg",
                category, self.shift_id, i
            ));
        }

        self.next_page();
    }
}