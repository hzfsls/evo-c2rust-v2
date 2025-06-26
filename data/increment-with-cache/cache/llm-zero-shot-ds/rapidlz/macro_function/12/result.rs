macro_rules! RAPIDLZ_UNLIKELY {
    ($x:expr) => {
        #[cold]
        if !$x { } else { }
    };
}

pub(crate) use RAPIDLZ_UNLIKELY;
