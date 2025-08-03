use godot::{
    classes::{ResourceLoader, Tween},
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

pub(crate) fn change_scene(node: &Node, scene: &str, should_trans: bool) {
    let path = get_path(&format!("scenes/{scene}.tscn"));
    if should_trans {
        let trans = "color_fade".to_variant();
        let mut manager = node.get_node_as::<Node>("/root/SceneTransitioner");
        let mut args = Dictionary::new();
        {
            let mut r#in = Dictionary::new();
            let _ = r#in.insert("duration", 0.5);
            let _ = args.insert("in", r#in);
        }
        {
            let mut out = Dictionary::new();
            let _ = out.insert("duration", 1.0);
            let _ = args.insert("out", out);
        }
        manager.call(
            "transition_to",
            &[path.to_variant(), trans.clone(), trans, args.to_variant()],
        );
    } else {
        node.get_tree()
            .as_mut()
            .map(|tree| tree.change_scene_to_file(&path));
    }
}

pub(crate) fn access<T: GodotClass, U, F>(node: &mut Option<Gd<T>>, mapper: F) -> Option<U>
where
    F: FnOnce(&mut Gd<T>) -> U,
{
    node.as_mut().map(mapper)
}

pub(crate) fn animate_position<T>(node: &mut Gd<T>, to: Vector2, duration: f64) -> Option<Gd<Tween>>
where
    T: GodotClass + Inherits<Node>,
{
    let mut node = node.clone().upcast::<Node>();
    let tween = node.create_tween();
    tween.map(|mut tween| {
        tween.tween_property(&node, "position", &to.to_variant(), duration);
        tween
    })
}
