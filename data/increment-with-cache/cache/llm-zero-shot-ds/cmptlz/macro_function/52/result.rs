macro_rules! CMPTLZ_READ32BIT {
    ($ptr:expr) => {
        (unsafe { ($ptr as *const CmptlzUnalignU32).as_ref().unwrap().v })
    };
}
pub(crate) use CMPTLZ_READ32BIT;
