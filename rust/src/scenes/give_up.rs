use crate::{
    helpers::{access, change_scene},
    player::AudioPlayer,
};
use godot::{classes::TextureButton, prelude::*};
use rand::random;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct GiveUp {
    base: Base<Node>,

    #[export]
    sfx: Option<Gd<AudioPlayer>>,
    #[export]
    no: Option<Gd<TextureButton>>,
}

#[godot_api]
impl GiveUp {
    #[func]
    fn accept(&mut self) {
        change_scene(&self.base(), "credits", true)
    }

    #[func]
    fn unaccept(&mut self) {
        // teleport the button
        let viewport = self
            .base()
            .get_viewport()
            .map(|v| v.get_visible_rect())
            .expect("viewport should be visible");

        access(&mut self.no, |button| {
            // change the scale
            let scale_factor = 0.25 + random::<f32>();
            button.set_scale(Vector2::new(scale_factor, scale_factor));

            // move the button to a random position within the viewport
            let button_size = button.get_size() * scale_factor;
            let max_x = (viewport.size.x - button_size.x).max(0.0);
            let max_y = (viewport.size.y - button_size.y).max(0.0);

            let x = random::<f32>() * max_x;
            let y = random::<f32>() * max_y;
            button.set_position(Vector2::new(x, y));
        });

        // play the sound effect
        access(&mut self.sfx, |player| {
            let i = rand::random_range(0..=3);
            player.bind_mut().play(&format!("sfx/give-up/{}", i));
        });
    }
}
