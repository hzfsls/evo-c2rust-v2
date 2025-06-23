macro_rules! RAPIDLZ_READ32BIT {
    ($ptr:expr) => {
        (unsafe { std::ptr::read_unaligned($ptr as *const u32) })
    };
}
pub(crate) use RAPIDLZ_READ32BIT;
