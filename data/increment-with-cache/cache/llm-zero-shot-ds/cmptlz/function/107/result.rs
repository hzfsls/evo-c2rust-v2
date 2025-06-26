use std::ptr;

#[inline]
fn cmpt_lz_try_dec_len_and_dist(
    dec_ctx: &mut CmptLzDecCtx,
    mk_state: u32,
    range: u32,
    range_code: u32,
    range_bound: u32,
    prob_slot: *mut CmptLzDecProb,
    buf_try_dec: *const u8,
    pbuf_limit: *mut *const u8,
) -> i32 {
    let pb_mask = ((1u32) << (dec_ctx.prop.pos_bits)) - 1;
    let pos_state = CMPTLZ_CALC_POS_STATE(dec_ctx.processed_pos, pb_mask);
    let buf_limit = unsafe { *pbuf_limit };
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
    let mut prob_len = unsafe { prob_slot.add(CMPTLZ_LEN_CHOICE) };
    let mut range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (unsafe { *prob_len });
    let (mut bits_2_be_dec, mut offset) = if range_code < range_bound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
        prob_len = unsafe { prob_slot.add(CMPTLZ_LOW_LENPROB_OFFSET + pos_state) };
        (3, 0)
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);

        prob_len = unsafe { prob_slot.add(CMPTLZ_LEN_CHOICE2) };
        range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (unsafe { *prob_len });
        if range_code < range_bound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
            prob_len = unsafe { prob_slot.add(CMPTLZ_LEN_CHOICE + CMPTLZ_LEN_CHOICE2 + pos_state) };
            (3, (CMPTLZ_LOW_LEN_CLASS << 1))
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
            prob_len = unsafe { prob_slot.add(CMPTLZ_HIGH_LENPROB_OFFSET) };
            (8, (CMPTLZ_LOW_LEN_CLASS << 1))
        }
    };

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);

    let mut dec_sym = 1;
    loop {
        let prob_bit = unsafe { prob_len.add(dec_sym) };
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, range_code, range_bound, dec_sym, prob_bit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
        if dec_sym >= ((1u32) << bits_2_be_dec) {
            break;
        }
    }
    dec_sym -= ((1u32) << bits_2_be_dec);
    dec_sym += offset;

    if mk_state >= 4 {
        unsafe { *pbuf_limit = buf_try_dec };
        return CMPT_OK;
    }

    prob_slot = cmpt_lz_get_pos_slot_prob(probs_matrix).add(cmpt_lz_get_len_condition(dec_sym));

    dec_sym = 1;
    loop {
        let prob_bit = unsafe { prob_slot.add(dec_sym) };
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, range_code, range_bound, dec_sym, prob_bit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
        if dec_sym >= (1 << CMPTLZ_POS_SLOT_BITS) {
            break;
        }
    }
    dec_sym -= (1 << CMPTLZ_POS_SLOT_BITS);

    let mut bits_2_be_dec = ((dec_sym >> 1) - 1);
    if dec_sym >= CMPTLZ_LOW_POSSLOT {
        if dec_sym < CMPTLZ_HIGH_POSSLOT {
            prob_slot = cmpt_lz_get_spec_pos_prob(probs_matrix)
                .add((cmpt_lz_get_base_dist_by_pos_slot(dec_sym) << bits_2_be_dec));
        } else {
            bits_2_be_dec -= CMPTLZ_LARGE_DIST_LOW_BITS;
            while bits_2_be_dec > 0 {
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
                range >>= 1;
                range_code -= range & (((range_code - range) >> 31) - 1);
                bits_2_be_dec -= 1;
            }
            prob_slot = cmpt_lz_get_ailgn_prob(probs_matrix);
            bits_2_be_dec = CMPTLZ_LARGE_DIST_LOW_BITS;
        }

        dec_sym = 1;
        let mut offset = 1;
        while bits_2_be_dec > 0 {
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
            let prob_bit = unsafe { prob_slot.add(dec_sym) };
            range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (unsafe { *prob_bit });
            if range_code < range_bound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
                dec_sym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
                offset <<= 1;
                dec_sym += offset;
            }
            bits_2_be_dec -= 1;
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
    unsafe { *pbuf_limit = buf_try_dec };
    CMPT_OK
}
