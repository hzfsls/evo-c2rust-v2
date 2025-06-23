macro_rules! CMPTLZ_UNLIKELY {
    ($expr:expr) => {
        #[cold]
        if !$expr {}
    };
}

pub(crate) use CMPTLZ_UNLIKELY;
