use crate::helpers::{access, change_scene};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Menu {
    base: Base<Node>,
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        change_scene(&self.base(), "intro");
    }

    #[func]
    fn quit_game(&self) {
        let mut tree = self.base().get_tree();
        access(&mut tree, |tree| {
            tree.quit();
        });
    }
}
