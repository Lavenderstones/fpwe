use godot::{classes::Button, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Controls {
    base: Base<Node>,
}

#[godot_api]
impl INode for Controls {
    fn ready(&mut self) {
        let approve = self.base().get_node_as::<Button>("Approve");
        let deny = self.base().get_node_as::<Button>("Deny");

        approve.signals().pressed().connect_other(self, on_approve);
        deny.signals().pressed().connect_other(self, on_deny);
    }
}

fn on_approve(controls: &mut Controls) {
    godot_print!("approved!");
}

fn on_deny(controls: &mut Controls) {
    godot_print!("denied!");
}
