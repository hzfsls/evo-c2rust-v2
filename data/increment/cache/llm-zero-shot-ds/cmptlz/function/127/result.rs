use std::ptr::write_unaligned;

#[inline]
fn cmptlz_write_le32bit(addr: *mut u32, val: u32) {
    if cmptlz_is_le() != 0 {
        unsafe {
            write_unaligned(addr, val);
        }
    } else {
        unsafe {
            write_unaligned(addr, cmptlz_swap32(val));
        }
    }
}
