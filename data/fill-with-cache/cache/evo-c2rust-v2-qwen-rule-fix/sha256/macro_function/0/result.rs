macro_rules! BITSIZE { ($t:ty) => { c_sizeof!($t) * BITS_PRE_BYTE!() } }
pub(crate) use BITSIZE;