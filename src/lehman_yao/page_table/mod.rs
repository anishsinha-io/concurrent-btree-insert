use rand::Rng;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

///
/// PageTable is a type alias for a thread-safe HashMap protected by a RwLock
///
type PageTable = Arc<RwLock<HashMap<i32, usize>>>;

///
/// Simple function to create a new PageTable
///
fn page_table() -> PageTable {
    Arc::new(RwLock::new(HashMap::new()))
}
