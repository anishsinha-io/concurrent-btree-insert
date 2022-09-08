use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Derivative, PartialEq, PartialOrd, Eq, Ord,
)]
#[derivative(Default)]
pub struct ItemPtr {
    #[derivative(Default(value = "-1"))]
    page_no: i32,
    #[derivative(Default(value = "0"))]
    offset: usize,
}

impl ItemPtr {
    pub fn new(page_no: i32, offset: usize) -> Self {
        ItemPtr { page_no, offset }
    }
}

impl Display for ItemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ItemPtr [page_no={} offset={}]",
            self.page_no, self.offset
        )
    }
}
