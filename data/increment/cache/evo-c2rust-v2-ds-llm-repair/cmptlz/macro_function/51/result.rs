macro_rules! CMPTLZ_WRITE32BIT { ($ptr:expr, $val:expr) => { ($ptr.cast::<Ptr<CmptlzUnalignU32>>()).v = $val } }
pub(crate) use CMPTLZ_WRITE32BIT;
