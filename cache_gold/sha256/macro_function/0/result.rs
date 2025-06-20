macro_rules! BITSIZE {
    ($t:ty) => {
        (std::mem::size_of::<$t>() * BITS_PRE_BYTE!())
    };
}
pub(crate) use BITSIZE;