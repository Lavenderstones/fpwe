use godot::prelude::*;

mod page;

struct FastPacedWorkingEnvironment;

#[gdextension]
unsafe impl ExtensionLibrary for FastPacedWorkingEnvironment {}
