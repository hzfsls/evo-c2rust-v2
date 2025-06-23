pub fn cmpt_price_len(len_encoder: &CmptLenEncoder, len: u32, pos_state: u32) -> u32 {
    len_encoder.prices[pos_state as usize][(len - CMPTLZ_MATCH_LEN_MIN) as usize]
}
