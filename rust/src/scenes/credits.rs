use godot::{classes::{AnimatedSprite2D, Label}, prelude::*};

use crate::helpers::{access, animate_position};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Credits {
    base: Base<Node>,

    #[export]
    rain_anim: Option<Gd<AnimatedSprite2D>>,

    #[export]
    text: Option<Gd<Label>>,
}

#[godot_api]
impl INode for Credits {
    fn ready(&mut self) {
        access(&mut self.rain_anim, |sprite| {
            sprite.play();
        });

        access(&mut self.text, |text| {
            let mut node = text.clone().upcast::<Node>().cast::<Node2D>();

            let from = node.get_position();

            let mut dest = node.get_position();
            dest.y += 3400.0;

            animate_position(
                &mut node,
                from,
                dest,
                60.0
            ).map(
                |mut tween: Gd<godot::classes::Tween>| {
                    tween.play()
                },
            );
        });
    }
}