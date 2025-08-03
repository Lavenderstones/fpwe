use crate::helpers::{access, change_scene};
use godot::{
    classes::{DisplayServer, display_server::WindowMode},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Menu {
    base: Base<Node>,
}

#[godot_api]
impl Menu {
    #[func]
    fn start_game(&self) {
        change_scene(&self.base(), "intro", true);
    }

    #[func]
    fn quit_game(&self) {
        let mut tree = self.base().get_tree();
        access(&mut tree, |tree| {
            tree.quit();
        });
    }

    #[func]
    fn toggle_fullscreen(&self) {
        let mut server = DisplayServer::singleton();
        let mode = server.window_get_mode();
        let new_mode = if mode == WindowMode::FULLSCREEN {
            WindowMode::WINDOWED
        } else {
            WindowMode::FULLSCREEN
        };
        server.window_set_mode(new_mode);
    }
}
