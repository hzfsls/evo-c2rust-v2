macro_rules! RAPIDLZ_READ32BIT {
    ($ptr:expr) => {
        (unsafe { &*($ptr as *const RapidlzUnalignU32) }.v)
    };
}

pub(crate) use RAPIDLZ_READ32BIT;
