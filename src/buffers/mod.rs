///
/// This module contains definitions and methods to work with thread-safe buffer pool objects
///
pub mod encoding;
pub mod fsutil;
pub mod node;
pub mod ptr;

use std::fmt::Display;
use std::sync::{Arc, Mutex, RwLock};

/// A page is a slice of 512 contiguous bytes.  
type Page = [u8; 512];

/// Buffer pool frames contain metadata about pages held in memory as well as the page itself. The main things that buffer pool frames
/// keep track of are:
///     - `dirty` (whether a page has been modified since it was read)
///     - `pins` (the number of times this page has been accessed since it's been in memory)
///     - `page_no` (the page number of the disk page held in the frame)
///     - `page` (the actual page in memory)
///
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

///
/// BufferPool is a type alias for a thread-safe vector of thread-safe frames
///
type BufferPool = Arc<RwLock<Vec<Arc<Mutex<BufferPoolFrame>>>>>;
