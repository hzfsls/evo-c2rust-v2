use std::mem;

fn rapidlz_write_le16(addr: &mut [u8], val: u16) {
    if cfg!(target_endian = "little") {
        let val_bytes = val.to_le_bytes();
        addr[..2].copy_from_slice(&val_bytes);
    } else {
        addr[0] = val as u8;
        addr[1] = (val >> 8) as u8;
    }
}
