use std::fmt;
use crate::Page;
use super::data;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum Sanity {
    #[default]
    Normal,
    Tired,
    Hell,
}

impl Sanity {
    pub(crate) fn pages(&self) -> &[Page] {
        match self {
            Sanity::Normal => &data::NORMAL,
            Sanity::Tired => &data::TIRED,
            Sanity::Hell => &data::HELL,
        }
    }
}

impl fmt::Display for Sanity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sanity::Normal => write!(f, "normal"),
            Sanity::Tired => write!(f, "tired"),
            Sanity::Hell => write!(f, "hell"),
        }
    }
}