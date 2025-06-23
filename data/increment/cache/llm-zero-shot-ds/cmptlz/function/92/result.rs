fn cmpt_lz_len_dec(
    dec_ctx: &mut CmptLzDecCtx,
    prob_slot: &mut [CmptLzDecProb],
    p_range: &mut u32,
    p_range_code: &mut u32,
    p_range_bound: &mut u32,
    pos_state: u32,
) -> u32 {
    let mut dec_len = 1;
    let mut range = *p_range;
    let mut range_code = *p_range_code;
    let mut range_bound = *p_range_bound;
    let mut buf_to_dec = dec_ctx.buf;
    let mut prob_len = &mut prob_slot[CMPTLZ_LEN_CHOICE as usize];

    let mut i = 0;
    if cmpt_lz_is_the_bit_0(prob_len, range, range_code, range_bound) {
        cmpt_lz_range_update_0(prob_len, &mut range, &mut range_bound);
        cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
        prob_len = &mut prob_slot[(CMPTLZ_LOW_LENPROB_OFFSET + pos_state) as usize];
        for _ in 0..CMPTLZ_LOW_LEN_BIT {
            cmpt_lz_len_bit_dec(
                &mut prob_len[dec_len as usize],
                &mut range,
                &mut range_code,
                &mut range_bound,
                &mut dec_len,
                &mut buf_to_dec,
            );
        }
        dec_len -= 8;
    } else {
        cmpt_lz_range_update_1(prob_len, &mut range, &mut range_code, &mut range_bound);
        cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);

        prob_len = &mut prob_slot[CMPTLZ_LEN_CHOICE2 as usize];
        if cmpt_lz_is_the_bit_0(prob_len, range, range_code, range_bound) {
            cmpt_lz_range_update_0(prob_len, &mut range, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);

            prob_len = &mut prob_slot[(CMPTLZ_LEN_CHOICE2 + pos_state) as usize];
            for _ in 0..CMPTLZ_LOW_LEN_BIT {
                cmpt_lz_len_bit_dec(
                    &mut prob_len[dec_len as usize],
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                    &mut dec_len,
                    &mut buf_to_dec,
                );
            }
        } else {
            cmpt_lz_range_update_1(prob_len, &mut range, &mut range_code, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);

            prob_len = &mut prob_slot[CMPTLZ_HIGH_LENPROB_OFFSET as usize];
            for _ in 0..CMPTLZ_HIGH_LEN_BIT {
                cmpt_lz_len_bit_dec(
                    &mut prob_len[dec_len as usize],
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                    &mut dec_len,
                    &mut buf_to_dec,
                );
            }
            dec_len -= CMPTLZ_HIGH_LEN_CLASS;
            dec_len += CMPTLZ_LOW_LEN_CLASS << 1;
        }
    }

    *p_range = range;
    *p_range_code = range_code;
    *p_range_bound = range_bound;
    dec_ctx.buf = buf_to_dec;

    dec_len
}
