use std::mem;

fn rapidlz_write_le16(addr: &mut [u8], val: u16) {
    if is_little_endian() {
        // SAFETY: We ensure the slice is at least 2 bytes long.
        unsafe {
            *(addr.as_mut_ptr() as *mut u16) = val;
        }
    } else {
        addr[0] = val as u8;
        addr[1] = (val >> 8) as u8;
    }
}

#[inline]
fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}
