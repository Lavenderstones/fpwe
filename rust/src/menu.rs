use crate::player::AudioPlayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct Menu {
    base: Base<Node>,

    #[export]
    player: Option<Gd<AudioPlayer>>,
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        self.base()
            .get_tree()
            .as_mut()
            .map(|tree| tree.change_scene_to_file(crate::shift::SCENE_URL));
    }

    #[func]
    fn quit_game(&self) {
        self.base().get_tree().as_mut().map(|tree| tree.quit());
    }
}
