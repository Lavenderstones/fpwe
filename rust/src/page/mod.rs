mod data;

#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) accept: bool,

    pub(crate) bonus: i32,
    pub(crate) penalty: i32,
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
