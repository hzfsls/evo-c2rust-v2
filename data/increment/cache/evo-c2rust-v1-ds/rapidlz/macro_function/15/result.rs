macro_rules! RAPIDLZ_READ64BIT { ($ptr:expr) => { (*$ptr.cast::<Ptr<RapidlzUnalignU64>>()).v } }
pub(crate) use RAPIDLZ_READ64BIT;
