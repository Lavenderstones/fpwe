use crate::page::{Page, SHIFT1};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node)]
struct Game {
    base: Base<Node>,
    current_page: Page,
    pages_left: Vec<Page>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        let mut game = Self {
            base,
            current_page: Page::new(false), // placeholder
            pages_left: SHIFT1.to_vec(),
        };
        game.next_page();
        game
    }
}

impl Game {
    fn next_page(&mut self) {
        if self.pages_left.is_empty() {
            // todo: next shift
            self.pages_left = SHIFT1.to_vec();
        }
        let i = rand::random_range(0..self.pages_left.len());
        self.current_page = self.pages_left.remove(i);
    }
}

#[godot_api]
impl Game {
    #[func]
    fn handle_choice(&mut self, accept: bool) {
        if accept == self.current_page.accept {
            godot_print!("praise");
            self.next_page();
        } else {
            godot_print!("scold");
        }
    }
}
