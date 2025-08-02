use std::marker::Destruct;

use godot::{
    classes::{Engine, ResourceLoader},
    prelude::*,
};

fn get_path(path: &str) -> String {
    format!("res://{}", path)
}

pub(crate) fn get_asset<T>(path: &str) -> Gd<T>
where
    T: GodotClass + Inherits<Resource>,
{
    ResourceLoader::singleton()
        .load(&get_path(&format!("assets/{path}")))
        .and_then(|res| res.try_cast::<T>().ok())
        .unwrap_or_else(|| panic!("Failed to load asset: {}", path))
}

pub(crate) fn get_state(node: &Node) -> Gd<Node> {
    node.get_tree()
        .unwrap()
        .get_root()
        .unwrap()
        .get_node_as::<Node>("State")
}

pub(crate) fn change_scene(node: &Node, scene: &str) {
    node.get_tree()
        .as_mut()
        .map(|tree| tree.change_scene_to_file(&get_path(&format!("scenes/{scene}.tscn"))));
}

pub(crate) fn access<T: GodotClass, U, F>(node: &mut Option<Gd<T>>, mapper: F) -> Option<U>
where
    F: FnOnce(&mut Gd<T>) -> U,
{
    node.as_mut().map(mapper)
}
