pub mod encoding;
pub mod node;
pub mod ptr;

use std::fmt::Display;
use std::sync::{Arc, Mutex, RwLock};

type Page = [u8; 512];

pub struct BufferPoolFrame {
    dirty: bool,
    pins: i32,
    page_no: i32,
    page: [u8; 512],
}

impl BufferPoolFrame {
    fn new(page_no: i32, page: [u8; 512]) -> Self {
        BufferPoolFrame {
            dirty: false,
            pins: 0,
            page_no,
            page,
        }
    }
}

impl Display for BufferPoolFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BufferPoolFrame [dirty={} pins={} page_no={} page_size={}]",
            self.dirty,
            self.pins,
            self.page_no,
            std::mem::size_of_val(&self.page)
        )
    }
}

impl PartialEq for BufferPoolFrame {
    fn eq(&self, other: &Self) -> bool {
        self.page_no == other.page_no
    }
}

type BufferPool = Vec<Mutex<BufferPoolFrame>>;
