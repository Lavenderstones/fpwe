use godot::{classes::{AudioStream, AudioStreamPlayer2D, ResourceLoader}, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct Menu {
    base: Base<Node>,
    #[export]
    music_player: Option<Gd<AudioStreamPlayer2D>>,
    music: Gd<AudioStream>
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        if let Some(tree) = self.base().get_tree().as_mut() {
            tree.change_scene_to_file("res://scenes/game.tscn");
        }
    }

    #[func]
    fn on_music_stop(&mut self) {
        self.music_player
        .as_mut()
        .map(|player| {
            let music = ResourceLoader::singleton()
            .load("res://assets/music/Stage0.ogg")
            .and_then(|res| res.try_cast::<AudioStream>().ok())
            .expect("music must be valid");

            player.stop();
            player.set_stream(&music);
            player.play();
        });
    }
}
