macro_rules! RAPIDLZ_READ32BIT { ($ptr:expr) => { 
    ((const RapidlzUnalignU32::cast::<Ptr<_>>($ptr)).v) 
} }
pub(crate) use RAPIDLZ_READ32BIT;