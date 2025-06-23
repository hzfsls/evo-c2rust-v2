macro_rules! CMPTLZ_WRITE32BIT { ($ptr:expr, $val:expr) => {
    {
        let mut unaligned_ptr = $ptr.cast::<Ptr<CmptlzUnalignU32>>();
        unaligned_ptr.v = $val;
    }
} }
pub(crate) use CMPTLZ_WRITE32BIT;