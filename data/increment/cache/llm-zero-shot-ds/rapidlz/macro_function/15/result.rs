macro_rules! RAPIDLZ_READ64BIT {
    ($ptr:expr) => {
        (unsafe { (*(($ptr as *const _ as *const RapidlzUnalignU64)).v })
    };
}
pub(crate) use RAPIDLZ_READ64BIT;
