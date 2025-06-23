macro_rules! cmptlz_find_min {
    ($x:expr, $y:expr) => {
        if $x < $y { $x } else { $y }
    };
}

pub(crate) use cmptlz_find_min;
