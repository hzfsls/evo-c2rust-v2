use std::io::{Read, Seek, SeekFrom};

fn bzp_file_eof(f: &mut std::fs::File) -> std::io::Result<bool> {
    let mut buf = [0u8; 1];
    match f.read(&mut buf) {
        Ok(0) => Ok(true), // EOF reached
        Ok(_) => {
            // Move the file position back by 1 byte
            f.seek(SeekFrom::Current(-1))?;
            Ok(false)
        }
        Err(e) => Err(e),
    }
}
