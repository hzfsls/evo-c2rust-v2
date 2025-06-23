macro_rules! RAPIDLZ_WRITE64BIT { ($ptr:expr, $val:expr) => { ($ptr.cast::<Ptr<RapidlzUnalignU64>>()).v = $val } }
pub(crate) use RAPIDLZ_WRITE64BIT;
