pub const SHIFT1: [Page; 2] = [Page::new(true), Page::new(false)];

#[derive(Clone)]
pub struct Page {
    pub accept: bool,
}

impl Page {
    pub const fn new(accept: bool) -> Self {
        Page { accept }
    }
}
