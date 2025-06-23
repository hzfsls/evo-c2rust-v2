macro_rules! CMPTLZ_READ32BIT { ($ptr:expr) => { 
    ((const CmptlzUnalignU32 { v: 0 }) as *const CmptlzUnalignU32).cast::<Ptr<CmptlzUnalignU32>>().read($ptr).v 
} }
pub(crate) use CMPTLZ_READ32BIT;