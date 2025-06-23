macro_rules! RAPIDLZ_READ64BIT {
    ($ptr:expr) => {
        (unsafe { (*($ptr as *const u64)).to_le() })
    };
}
pub(crate) use RAPIDLZ_READ64BIT;
