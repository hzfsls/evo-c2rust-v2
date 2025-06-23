macro_rules! RAPIDLZ_LIKELY {
    ($x:expr) => {
        if $x { true } else { false }
    };
}
pub(crate) use RAPIDLZ_LIKELY;
