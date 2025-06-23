macro_rules! cmptlz_single_bit_try_dec {
    ($range:expr, $range_code:expr, $range_bound:expr, $dec_sym:expr, $prob_sym:expr) => {
        $range_bound = ($range >> CMPTLZ_PROB_LG_BIT) * (*$prob_sym);
        if $range_code < $range_bound {
            cmptlz_range_update_after_dec_bit0!($range, $range_bound);
            $dec_sym = $dec_sym << 1;
        } else {
            cmptlz_range_update_after_dec_bit1!($range, $range_code, $range_bound);
            $dec_sym = ($dec_sym << 1) + 1;
        }
    };
}

pub(crate) use cmptlz_single_bit_try_dec;
