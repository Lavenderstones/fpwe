use crate::shift::page::Page;

pub(crate) const SHIFT1: [Page; 2] = [
    Page {
        title: "Page 1",
        description: "Woahg",
        accept: true,
        bonus: 10,
        penalty: 100,
    },
    Page {
        title: "Page 2",
        description: "Oh no",
        accept: false,
        bonus: 100,
        penalty: 10,
    },
];
