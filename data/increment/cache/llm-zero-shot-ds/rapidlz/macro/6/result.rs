macro_rules! RAPIDLZ_ERROR_OUTPUT {
    ($curSrc:expr, $src:expr) => {
        -($curSrc as isize - $src as isize) - 1
    };
}

pub(crate) use RAPIDLZ_ERROR_OUTPUT;
