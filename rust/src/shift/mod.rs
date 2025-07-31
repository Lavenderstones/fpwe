use page::Page;

mod page;

pub(crate) const SHIFT1_PAGES: [Page; 2] = [
    Page::new(true, 10, 100, "page1.jpg"),
    Page::new(false, 100, 10, "page2.jpg"),
];

pub struct Shift {
    left: Vec<Page>,
    pub(crate) current: Page,
}

/// Select a random [Page] from the provided list and remove it.
/// Returns `None` if the list is empty.
fn select_page(pages: &mut Vec<Page>) -> Option<Page> {
    if pages.is_empty() {
        return None;
    }
    let i = rand::random_range(0..pages.len());
    Some(pages.remove(i))
}

impl<const N: usize> From<[Page; N]> for Shift {
    fn from(pages: [Page; N]) -> Self {
        let mut pages = pages.to_vec();
        let current = select_page(&mut pages).expect("a shift must have at least one page");
        Shift {
            left: pages,
            current,
        }
    }
}

impl Shift {
    /// Move to the next [Page] in the [Shift].
    pub(crate) fn next_page(&mut self) -> Option<Page> {
        select_page(&mut self.left)
    }

    /// Is the [Shift] done?
    pub(crate) fn is_done(&self) -> bool {
        self.left.is_empty()
    }

    /// The current [Page] in the [Shift].
    pub(crate) fn page(&self) -> &Page {
        &self.current
    }
}
