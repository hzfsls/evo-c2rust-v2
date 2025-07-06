macro_rules! RAPIDLZ_READ32BIT { ($ptr:expr) => { $ptr.cast::<Ptr<RapidlzUnalignU32>>().v } }
pub(crate) use RAPIDLZ_READ32BIT;
