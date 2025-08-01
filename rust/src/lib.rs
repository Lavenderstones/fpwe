use godot::prelude::*;

mod game;
mod menu;
mod player;
mod shift;

pub(crate) fn assets(path: &str) -> String {
    format!("res://assets/{}", path)
}

struct FPWE;

#[gdextension]
unsafe impl ExtensionLibrary for FPWE {}
