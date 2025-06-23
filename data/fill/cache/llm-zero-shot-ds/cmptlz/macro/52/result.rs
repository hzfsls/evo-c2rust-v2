macro_rules! CMPTLZ_ALIGN_TABLE_SIZE {
    () => {
        1 << $crate::CMPTLZ_LARGE_DIST_LOW_BITS
    };
}
pub(crate) use CMPTLZ_ALIGN_TABLE_SIZE;
