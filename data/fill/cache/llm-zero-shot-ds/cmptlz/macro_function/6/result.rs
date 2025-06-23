macro_rules! cmptlz_calc_pos_state {
    ($procPos:expr, $pbMask:expr) => {
        (($procPos) & ($pbMask)) << 4
    };
}
pub(crate) use cmptlz_calc_pos_state;
