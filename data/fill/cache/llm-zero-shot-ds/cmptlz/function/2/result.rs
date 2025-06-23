use std::ptr::write_unaligned;

fn cmptlz_write_le32bit(addr: *mut u32, val: u32) {
    if cmptlz_is_le() {
        unsafe {
            write_unaligned(addr, val);
        }
    } else {
        unsafe {
            write_unaligned(addr, cmptlz_swap32(val));
        }
    }
}
