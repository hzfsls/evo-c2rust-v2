macro_rules! CMPTLZ_WRITE32BIT {
    ($ptr:expr, $val:expr) => {
        (unsafe { &mut *($ptr as *mut CmptlzUnalignU32) }.v = $val)
    };
}
pub(crate) use CMPTLZ_WRITE32BIT;
