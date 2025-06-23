macro_rules! RAPIDLZ_WRITE64BIT {
    ($ptr:expr, $val:expr) => {
        (unsafe { &mut *($ptr as *mut RapidlzUnalignU64) }.v = $val)
    };
}
pub(crate) use RAPIDLZ_WRITE64BIT;
