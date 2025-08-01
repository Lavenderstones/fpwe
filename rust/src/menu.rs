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
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.change_scene_to_file(crate::shift::SCENE_URL);
        }
    }

    #[func]
    fn on_music_stop(&mut self) {
        self.player.as_mut().map(|player| {
            player.bind_mut().play("music/menu.ogg");
        });
    }
}
