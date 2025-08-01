use crate::{player::AudioPlayer, shift::Shift};
use godot::{
    classes::{Label, ResourceLoader, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Game {
    base: Base<Node>,
    shift: Shift,
    score: isize,

    #[export]
    player: Option<Gd<AudioPlayer>>,

    #[export]
    score_label: Option<Gd<Label>>,
    #[export]
    page_sprite: Option<Gd<Sprite2D>>,
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        self.player
            .as_mut()
            .map(|player| player.bind_mut().play(self.shift.music));
    }
}

impl Game {
    fn next_page(&mut self) {
        if self.shift.is_done() {
            // todo: next shift
            self.shift = Shift::default();
        }
        self.shift.next_page();

        // set the texture
        if let Some(sprite) = self.page_sprite.as_mut() {
            let texture = ResourceLoader::singleton()
                .load(&self.shift.page().asset())
                .and_then(|res| res.try_cast::<Texture2D>().ok());
            if let Some(texture) = texture {
                sprite.set_texture(&texture);
            }
        }
    }

    fn update_score(&mut self, dx: isize) {
        self.score += dx;
        if let Some(label) = self.score_label.as_mut() {
            label.set_text(&format!("Score: {}", self.score));
        }
    }
}

#[godot_api]
impl Game {
    #[func]
    fn handle_choice(&mut self, answer: bool) {
        let page = self.shift.page();
        if page.check(answer) {
            godot_print!("praise");
            self.update_score(page.bonus as isize);
        } else {
            godot_print!("scold");
            self.update_score(-(page.penalty as isize));
        }

        self.next_page();
    }
}
