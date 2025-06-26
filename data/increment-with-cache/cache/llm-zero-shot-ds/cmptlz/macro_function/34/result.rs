macro_rules! cmptlz_range_normalize {
    ($range:expr, $range_code:expr, $buf_to_dec:expr) => {
        if $range < $crate::CMPTLZ_RANGE_DOWN_LIMIT {
            $range <<= $crate::CMPTLZ_ONE_BYTE_WIDTH;
            $range_code <<= $crate::CMPTLZ_ONE_BYTE_WIDTH;
            $range_code |= *$buf_to_dec;
            $buf_to_dec = $buf_to_dec.offset(1);
        }
    };
}

pub(crate) use cmptlz_range_normalize;
