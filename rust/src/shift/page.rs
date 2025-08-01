#[derive(Clone)]
pub(crate) struct Page {
    /// The title of the page.
    pub(crate) title: &'static str,
    /// A description of the page.
    pub(crate) description: &'static str,
    /// Whether the page should be accepted.
    pub(crate) accept: bool,
    /// The points you recieve for answering the page correctly.
    pub(crate) bonus: isize,
    /// The points you lose for answering the page incorrectly.
    pub(crate) penalty: isize,
}

impl Page {
    /// Check if the given answer is correct.
    ///
    /// Returns a tuple where the first element is a boolean indicating if the answer was correct,
    /// and the second element is the score change (bonus or penalty).
    pub(crate) fn check(&self, answer: bool) -> (bool, isize) {
        let correct = self.accept == answer;
        let dx = if correct { self.bonus } else { self.penalty };
        (correct, dx)
    }
}
