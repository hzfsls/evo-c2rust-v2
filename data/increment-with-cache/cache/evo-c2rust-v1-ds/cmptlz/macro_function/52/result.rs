macro_rules! CMPTLZ_READ32BIT { ($ptr:expr) => { (*$ptr.cast::<Ptr<CmptlzUnalignU32>>()).v } }
pub(crate) use CMPTLZ_READ32BIT;
