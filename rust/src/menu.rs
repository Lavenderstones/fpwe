use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct Menu {
    base: Base<Node>,
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.change_scene_to_file("res://scenes/game.tscn");
        }
    }
}
