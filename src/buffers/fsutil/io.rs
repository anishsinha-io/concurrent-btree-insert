use positioned_io::{ReadAt, WriteAt};
use serde::Serialize;

/**
 * <p>This method is used to write arbitraty bytes to a specified file.</p>
 *
 * @param abs_path the absolute path of the file
 * @param bytes the buffer to be written
 * @param offset the offset at which the bytes will be written
 *
 * @return Option<usize>
 */
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

/**
 * <p>This method is used to read bytes into a buffer from a file at a certain offset</p>
 *
 * @param abs_path the absolute path of the file
 * @param buffer the chunk of memory to be read to
 * @param offset the starting offset at which the bytes will be read
 *
 * @return ()
 */
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
