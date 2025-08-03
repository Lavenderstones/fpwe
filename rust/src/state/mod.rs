use godot::prelude::*;
pub(crate) use sanity::Sanity;
use std::collections::HashSet;

mod data;
mod sanity;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct State {
    base: Base<Node>,

    pub(crate) credits: isize,
    pub(crate) sanity: Sanity,
    pub(crate) fired_seen: HashSet<u8>,
}

impl State {
    pub(crate) fn get(node: &Node) -> Gd<Self> {
        node.get_node_as::<State>("/root/GameState")
    }

    pub(crate) fn next_shift(&mut self) {
        self.sanity = match self.sanity {
            Sanity::Normal => Sanity::Tired,
            Sanity::Tired => Sanity::Hell,
            Sanity::Hell => Sanity::Hell,
        };
    }

    pub(crate) fn reset(&mut self) {
        self.credits = 0;
        self.sanity = Sanity::default();
        self.fired_seen.clear();
    }
}
