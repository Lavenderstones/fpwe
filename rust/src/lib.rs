use godot::prelude::*;

mod game;
mod page;

struct FastPacedWorkingEnvironment;

#[gdextension]
unsafe impl ExtensionLibrary for FastPacedWorkingEnvironment {}
