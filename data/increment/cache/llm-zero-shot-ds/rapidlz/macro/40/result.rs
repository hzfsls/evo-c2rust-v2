#[allow(unused_macros)]
macro_rules! RAPIDLZ_FALLTHROUGH {
    () => {
        #[allow(unused_attributes)]
        #[fallthrough]
    };
}

pub(crate) use RAPIDLZ_FALLTHROUGH;
