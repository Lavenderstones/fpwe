pub(crate) fn get_shift1() -> Vec<Page> {
    vec![
        Page::new(true, 10, 100, "page1.jpg"),
        Page::new(false, 100, 10, "page2.jpg"),
    ]
}

#[derive(Clone)]
pub(crate) struct Page {
    accept: bool,
    pub(crate) asset: String,
    pub(crate) bonus: u8,
    pub(crate) penalty: u8,
}

impl Page {
    pub fn new(accept: bool, bonus: u8, penalty: u8, asset: &str) -> Self {
        Page {
            accept,
            bonus,
            penalty,
            asset: format!("res://assets/{}", asset),
        }
    }

    pub(crate) fn check(&self, answer: bool) -> bool {
        self.accept == answer
    }

    /// Select a random [Page] from the provided list and remove it.
    pub(crate) fn select(pages: &mut Vec<Page>) -> Page {
        let i = rand::random_range(0..pages.len());
        pages.remove(i)
    }
}
