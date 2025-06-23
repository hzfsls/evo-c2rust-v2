macro_rules! CMPTLZ_LIKELY {
    ($expr:expr) => {
        if $expr { true } else { false }
    };
}
pub(crate) use CMPTLZ_LIKELY;
