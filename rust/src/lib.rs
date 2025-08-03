use godot::prelude::*;

pub(crate) mod helpers;
mod player;
mod scenes;
mod state;

#[derive(Clone, Copy)]
pub(crate) struct Page {
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) accept: bool,

    pub(crate) bonus: isize,
    pub(crate) penalty: isize,
}


struct Fpwe;

#[gdextension]
unsafe impl ExtensionLibrary for Fpwe {}
