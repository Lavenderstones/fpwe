use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Page {
    base: Base<Node>,

    /// Whether the page should be accepted or not.
    #[export]
    accept: bool,
}

#[godot_api]
impl Page {
    #[func]
    fn handle_button(&mut self, accept: bool) {
        if self.accept == accept {
            godot_print!("praise");
        } else {
            godot_print!("scold");
        }
    }
}
