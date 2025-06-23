use std::mem;

fn rapidlz_read_le16_bit(addr: *const u8) -> u16 {
    if is_little_endian() {
        unsafe { *(addr as *const u16) }
    } else {
        let tmp1 = unsafe { *addr };
        let tmp2 = unsafe { *addr.add(1) };
        tmp1 as u16 | ((tmp2 as u16) << 8)
    }
}

#[inline]
fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}
