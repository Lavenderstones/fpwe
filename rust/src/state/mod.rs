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
        node.get_tree()
            .and_then(|tree| tree.get_root())
            .map(|root| root.get_node_as::<State>("GameState"))
            .expect("State should have been autoloaded")
    }

    pub(crate) fn next_shift(&mut self) {
        self.sanity = match self.sanity {
            Sanity::Normal => Sanity::Tired,
            Sanity::Tired => Sanity::Hell,
            Sanity::Hell => unreachable!(),
        };
    }
}
