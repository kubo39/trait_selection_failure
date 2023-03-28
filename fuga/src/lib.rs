//use std::convert::{TryInto, TryFrom};
pub use hoge::Hoge;

pub struct Fuga;

impl From<Hoge> for Fuga {
    fn from(_: Hoge) -> Self {
        Self {}
    }
}

impl From<Fuga> for Hoge {
    fn from(_: Fuga) -> Self {
        Self {}
    }
}
