macro_rules! cmpt_get_dist_state {
    ($len:expr) => {
        if $len < 4 + $crate::CMPTLZ_MATCH_LEN_MIN {
            $len - $crate::CMPTLZ_MATCH_LEN_MIN
        } else {
            4 - 1
        }
    };
}

pub(crate) use cmpt_get_dist_state;
