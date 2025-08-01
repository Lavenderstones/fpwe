use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Menu {
    base: Base<Node>
}

#[godot_api]
impl Menu {
    /// Start the game.
    #[func]
    fn start_game(&self) {
        self.base()
            .get_tree()
            .as_mut()
            .map(|tree| tree.change_scene_to_file("res://scenes/IntroCutscene.tscn"));
    }

    /// Quit the game.
    #[func]
    fn quit_game(&self) {
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.quit()
        }
    }
}
