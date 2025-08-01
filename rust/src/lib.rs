use godot::prelude::*;

mod game;
mod menu;
mod player;
mod shift;

struct FastPacedWorkingEnvironment;

#[gdextension]
unsafe impl ExtensionLibrary for FastPacedWorkingEnvironment {}
