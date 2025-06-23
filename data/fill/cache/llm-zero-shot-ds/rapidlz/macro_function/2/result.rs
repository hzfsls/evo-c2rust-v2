macro_rules! RAPIDLZ_UNLIKELY {
    ($x:expr) => {
        if !$x {
            core::intrinsics::unlikely($x)
        } else {
            false
        }
    };
}

pub(crate) use RAPIDLZ_UNLIKELY;
