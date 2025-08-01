use crate::{player::AudioPlayer, shift::Shift, assets};
use godot::{
    classes::{Label, ResourceLoader, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct Game {
    base: Base<Node>,
    shift: Shift,
    shift_number: usize,

    /// The current score of the player.
    score: isize,
    /// The label to display the score.
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
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        self.update();
    }
}

impl Game {
    /// Update fields in [Game] that depend on the current [Shift].
    fn update(&mut self) {
        // set the music
        self.music.as_mut().map(|player| {
            player.bind_mut().play(&format!("shift/{}.ogg", self.shift_number));
        });

        // set the page texture
        self.page.as_mut().map(|sprite| {
            ResourceLoader::singleton()
                .load(&assets(&format!("pages/{}/{}.webp", self.shift_number, self.shift.index)))
                .and_then(|res| res.try_cast::<Texture2D>().ok())
                .map(|texture| sprite.set_texture(&texture))
        });

        // set the page title
        let page = self.shift.page();
        self.title.as_mut().map(|label| {
            label.set_text(page.title);
        });

        // set the page description
        self.description.as_mut().map(|label| {
            label.set_text(page.description);
        });
    }

    fn next_page(&mut self) {
        if self.shift.is_done() {
            // todo: next shift
            self.shift = Shift::default();
        }
        self.shift.next();
        self.update();
    }
}

#[godot_api]
impl Game {
    #[func]
    fn handle_choice(&mut self, answer: bool) {
        let page = self.shift.page();

        // update score
        let (correct, dx) = page.check(answer);
        self.score += dx;
        if let Some(label) = self.score_label.as_mut() {
            label.set_text(&self.score.to_string());
        }

        // praise/scold
        if correct {
            godot_print!("praise");
        } else {
            godot_print!("scold");
        }

        self.next_page();
    }
}
