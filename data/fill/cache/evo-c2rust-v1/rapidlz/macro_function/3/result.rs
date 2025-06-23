macro_rules! RAPIDLZ_READ16BIT { ($ptr:expr) => { (*$ptr.cast::<Ptr<RapidlzUnalignU16>>()).v } }
pub(crate) use RAPIDLZ_READ16BIT;
