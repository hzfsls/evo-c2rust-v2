macro_rules! RAPIDLZ_READ16BIT {
    ($ptr:expr) => {
        (*(($ptr as *const u16)).to_le()
    };
}
pub(crate) use RAPIDLZ_READ16BIT;
