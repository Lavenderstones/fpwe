mod data;

#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) accept: bool,

    pub(crate) bonus: isize,
    pub(crate) penalty: isize,
}

impl Page {
    pub(crate) fn for_sanity(sanity: u8) -> &'static [Page] {
        match sanity {
            0 => &data::NORMAL,
            1 => &data::TIRED,
            2 => &data::INSANE,
            _ => unreachable!(),
        }
    }
}
