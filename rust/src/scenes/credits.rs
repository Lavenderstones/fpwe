use crate::{
    helpers::{access, animate_position, change_scene},
    state::State,
};
use godot::{
    classes::{AnimatedSprite2D, Label},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Credits {
    base: Base<Node>,

    #[export]
    rain: Option<Gd<AnimatedSprite2D>>,
    #[export]
    label: Option<Gd<Label>>,
}

#[godot_api]
impl INode for Credits {
    fn ready(&mut self) {
        access(&mut self.rain, |sprite| {
            sprite.play();
        });

        let base = self.base().clone();
        access(&mut self.label, |text| {
            let mut node = text.clone();
            let from = node.get_position();

            animate_position(
                &mut node,
                Vector2::new(from.x, from.y - text.get_size().y),
                64.05,
            )
            .map(|mut tween: Gd<godot::classes::Tween>| {
                tween.signals().finished().connect_self(move |_| {
                    // reset state
                    let mut state = State::get(&base);
                    state.bind_mut().reset();
                    // go to main menu
                    change_scene(&base, "menu", true);
                });
                tween.play()
            });
        });
    }
}
