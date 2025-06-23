macro_rules! cmptlz_likely {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            core::intrinsics::likely($expr)
        }
    };
}

pub(crate) use cmptlz_likely;
