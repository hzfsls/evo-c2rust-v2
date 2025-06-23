macro_rules! cmptlz_unlikely {
    ($expr:expr) => {
        #[cold]
        if !$expr {
            unreachable!();
        }
    };
}
pub(crate) use cmptlz_unlikely;
