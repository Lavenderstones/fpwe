#[derive(Clone)]
pub(crate) struct Page {
    accept: bool,
    asset: &'static str,
    pub(crate) bonus: u8,
    pub(crate) penalty: u8,
}

impl Page {
    pub const fn new(accept: bool, bonus: u8, penalty: u8, asset: &'static str) -> Self {
        Page {
            accept,
            bonus,
            penalty,
            asset,
        }
    }

    pub(crate) fn asset(&self) -> String {
        format!("res://assets/pages/{}", self.asset)
    }

    pub(crate) fn check(&self, answer: bool) -> bool {
        self.accept == answer
    }
}
