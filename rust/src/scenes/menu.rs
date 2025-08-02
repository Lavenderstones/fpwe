use crate::helpers::change_scene;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Menu {
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
        self.base().get_tree().as_mut().map(|tree| tree.quit());
    }
}
