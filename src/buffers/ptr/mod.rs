use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// ItemPtr is a structure which represents the logical location of an item on the disk. It's just a wrapper for an i32, but it could
/// be expanded to contain more information such as file metadata
///
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Derivative, PartialEq, PartialOrd, Eq, Ord,
)]
#[derivative(Default)]
pub struct ItemPtr {
    #[derivative(Default(value = "-1"))]
    page_no: i32,
}

impl ItemPtr {
    pub fn new(page_no: i32) -> Self {
        ItemPtr { page_no }
    }
}

impl Display for ItemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemPtr [page_no={}]", self.page_no)
    }
}
