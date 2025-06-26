use std::mem;

fn rapidlz_read_le16_bit(addr: &[u8]) -> u16 {
    if is_little_endian() {
        // SAFETY: addr is guaranteed to have at least 2 bytes since we're reading a u16
        unsafe { *(addr.as_ptr() as *const u16) }
    } else {
        let tmp1 = addr[0] as u16;
        let tmp2 = addr[1] as u16;
        tmp1 + (tmp2 << 8)
    }
}

#[inline]
fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}
