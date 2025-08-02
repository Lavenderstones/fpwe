use godot::prelude::*;

pub(crate) mod helpers;
mod player;
mod scenes;

struct Fpwe;

#[gdextension]
unsafe impl ExtensionLibrary for Fpwe {}
