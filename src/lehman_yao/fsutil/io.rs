///
/// This module contains implementations of `pwrite` and `pread` from C in Rust. The `pwrite` and `pread` functions are part of the POSIX
/// standard and are used for writing chunks of memory to file at specified offsets without having to seek first.  
///
use positioned_io::{ReadAt, WriteAt};
use serde::Serialize;

///
/// This function is used to write a memory buffer to a specified file at a specified offset
///
pub fn write_bytes(abs_path: &str, bytes: Vec<u8>, offset: u64) -> Option<usize> {
    use std::path::Path;
    let path = Path::new(abs_path);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();

    let bytes_written = file.write_at(offset, &bytes);
    match bytes_written {
        Err(e) => None,
        Ok(bytes_written) => Some(bytes_written),
    }
}

///
/// This function is used to read from a specified file, starting from a certain offset, enough bytes to fill the buffer passed in
///
pub fn read_bytes(abs_path: &str, buffer: &mut Vec<u8>, offset: u64) -> () {
    let path = std::path::Path::new(abs_path);
    let file = std::fs::OpenOptions::new()
        .read(true)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();
    file.read_at(offset, buffer).unwrap();
}
