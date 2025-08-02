#![feature(const_destruct)]

use godot::prelude::*;

pub(crate) mod helpers;
mod page;
mod player;
mod scenes;

struct Fpwe;

#[gdextension]
unsafe impl ExtensionLibrary for Fpwe {}
