use crate::player::AudioPlayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Menu {
    base: Base<Node>,

    /// The music player for the menu.
    #[export]
    music: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl Menu {
    /// Start the game.
    #[func]
    fn start_game(&self) {
        self.base()
            .get_tree()
            .as_mut()
            .map(|tree| tree.change_scene_to_file("res://scenes/shift.tscn"));
    }

    /// Quit the game.
    #[func]
    fn quit_game(&self) {
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.quit()
        }
    }
}
