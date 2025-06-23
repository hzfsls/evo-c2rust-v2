use std::io::{Read, Seek, SeekFrom};

fn bzp_file_eof(f: &mut std::fs::File) -> std::io::Result<bool> {
    let mut buf = [0u8; 1];
    match f.read(&mut buf) {
        Ok(0) => Ok(true), // EOF reached
        Ok(1) => {
            // Move the file position back by 1 byte
            f.seek(SeekFrom::Current(-1))?;
            Ok(false)
        }
        Err(e) => Err(e),
        _ => unreachable!(), // read() can only return 0 or 1 when reading 1 byte
    }
}
