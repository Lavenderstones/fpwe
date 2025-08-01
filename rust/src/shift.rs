use std::collections::HashSet;
use page::Page;
use rand::seq::IndexedRandom;
use crate::{shift::data::SHIFT1};

pub mod data;
mod page;

pub(crate) struct Shift {
    pages: &'static [Page],
    pub(crate) index: usize,
    seen: HashSet<usize>,
}

impl Default for Shift {
    fn default() -> Self {
        let pages = &SHIFT1;
        let index = rand::random_range(0..pages.len());
        Shift {
            pages,
            index,
            seen: HashSet::new(),
        }
    }
}

impl Shift {
    /// Get the current [Page] of the shift.
    pub(crate) fn page(&self) -> &Page {
        &self.pages[self.index]
    }

    /// Is the [Shift] done?
    pub(crate) fn is_done(&self) -> bool {
        self.seen.len() == self.pages.len()
    }

    /// Move to the next [Page] in the [Shift].
    pub(crate) fn next(&mut self) {
        let unseen: Vec<usize> = (0..self.pages.len())
            .filter(|i| !self.seen.contains(i))
            .collect();

        if !unseen.is_empty() {
            let &index = unseen
                .choose(&mut rand::rng())
                .expect("should have unseen pages");
            self.seen.insert(index);
            self.index = index;
        }
    }
}
