use crate::page::{Page, get_shift1};
use godot::{
    classes::{Label, ResourceLoader, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = Node)]
struct Game {
    base: Base<Node>,
    /// The current page being displayed.
    page: Page,
    /// The pages left in the current shift.
    pages_left: Vec<Page>,
    /// The player's score.
    score: isize,

    // -- subnodes --
    /// The label to display the score.
    #[export]
    score_label: Option<Gd<Label>>,
    #[export]
    page_sprite: Option<Gd<Sprite2D>>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        let mut pages_left = get_shift1();
        Self {
            base,
            page: Page::select(&mut pages_left),
            pages_left,
            score: 0,
            score_label: None,
            page_sprite: None,
        }
    }
}

impl Game {
    fn next_page(&mut self) {
        if self.pages_left.is_empty() {
            // todo: next shift
            self.pages_left = get_shift1();
        }
        self.page = Page::select(&mut self.pages_left);

        // set the texture
        if let Some(sprite) = self.page_sprite.as_mut() {
            let texture = ResourceLoader::singleton()
                .load(&self.page.asset)
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
        if self.page.check(answer) {
            godot_print!("praise");
            self.update_score(self.page.bonus as isize);
        } else {
            godot_print!("scold");
            self.update_score(-(self.page.penalty as isize));
        }

        self.next_page();
    }
}
