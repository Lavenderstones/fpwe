use crate::helpers::change_scene;
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
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.quit()
        }
    }
}
