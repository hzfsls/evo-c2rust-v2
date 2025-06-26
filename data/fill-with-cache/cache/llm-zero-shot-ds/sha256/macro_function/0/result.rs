macro_rules! BITSIZE {
    ($t:ty) => {
        (std::mem::size_of::<$t>() * $crate::BITS_PRE_BYTE)
    };
}
pub(crate) use BITSIZE;
