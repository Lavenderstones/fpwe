use godot::prelude::*;

mod game;
mod shift;

struct FastPacedWorkingEnvironment;

#[gdextension]
unsafe impl ExtensionLibrary for FastPacedWorkingEnvironment {}
