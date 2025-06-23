macro_rules! rapidlz_likely {
    ($x:expr) => {
        if $x { true } else { false }
    };
}
pub(crate) use rapidlz_likely;
