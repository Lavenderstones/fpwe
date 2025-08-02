use godot::prelude::*;

use crate::change_scene;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Menu {
    base: Base<Node>,
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        change_scene(&self.base(), "shift");
    }

    #[func]
    fn quit_game(&self) {
        self.base().get_tree().as_mut().map(|tree| tree.quit());
    }
}
