pub(crate) const SHIFT1: [Page; 2] = [Page::new(true, 10, 100), Page::new(false, 100, 10)];

#[derive(Clone)]
pub(crate) struct Page {
    accept: bool,
    pub(crate) bonus: u8,
    pub(crate) penalty: u8,
}

impl Page {
    /// Create a new [Page].
    pub(crate) const fn new(accept: bool, bonus: u8, penalty: u8) -> Self {
        Page {
            accept,
            bonus,
            penalty,
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
