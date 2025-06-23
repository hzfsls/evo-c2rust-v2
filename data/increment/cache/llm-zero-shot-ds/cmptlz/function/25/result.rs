use std::cmp::min;

const CMPTLZ_MATCH_LEN_MIN: u32 = 2;
const CMPT_DP_OPTMAX: u32 = 1 << 12;
const CMPTLZ_NUM_REPS: usize = 4;
const CMPT_INFINITY_PRICE: u32 = 0x0FFFFFFF;

#[inline]
fn cmptlz_dp_process(
    enc_ctx: &mut CmptLzEncCtx,
    mf: &mut CmptMfCtx,
    main_reps: &[u32; CMPTLZ_NUM_REPS],
    len_end: u32,
    position: u32,
    cur: u32,
) -> u32 {
    let cur_state = enc_ctx.opts[cur as usize].state;
    let buf_avail_full = min(CmptMfAvail(mf) + 1, CMPT_DP_OPTMAX - 1 - cur);
    let buf = CmptMfGetPtr(mf).offset(-1);
    let nice_len = mf.nice_len;
    let cur_price = enc_ctx.opts[cur as usize].price;
    let cur_byte = unsafe { *buf };
    let latest_match_byte = unsafe { *(buf.offset(-(main_reps[0] as isize) - 1)) };
    let pos_state = position & enc_ctx.pos_mask;

    enc_ctx.lit_marcov.pos = position;
    enc_ctx.lit_marcov.prev_byte = unsafe { *(buf.offset(-1)) };

    cmptlz_dp_try_cur_and_lit(
        enc_ctx,
        cur_price,
        cur_state,
        pos_state,
        cur,
        latest_match_byte,
        cur_byte,
    );

    let match_price = cur_price + cmpt_price_bit1(enc_ctx, enc_ctx.is_match[cur_state as usize][pos_state as usize]);
    let rep_match_price = match_price + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep[cur_state as usize]);

    if cur_byte == latest_match_byte
        && !(enc_ctx.opts[(cur + 1) as usize].pos_prev < cur
            && enc_ctx.opts[(cur + 1) as usize].back_prev == 0)
    {
        cmptlz_dp_try_cur_and_short(enc_ctx, rep_match_price, cur, cur_state, pos_state);
    }

    if buf_avail_full < CMPTLZ_MATCH_LEN_MIN {
        return len_end;
    }

    let buf_avail = min(buf_avail_full, nice_len);
    let mut start_len = CMPTLZ_MATCH_LEN_MIN;

    for main_rep_index in 0..CMPTLZ_NUM_REPS {
        let buf_rep_back = unsafe { buf.offset(-(main_reps[main_rep_index] as isize) - 1) };

        if unsafe { *buf != *buf_rep_back || *buf.offset(1) != *buf_rep_back.offset(1) } {
            continue;
        }

        let len_equal = cmpt_mem_cmp_len_safe(buf, buf_rep_back, CMPTLZ_MATCH_LEN_MIN, buf_avail);
        let mut new_len_end = len_end;
        while new_len_end < cur + len_equal {
            new_len_end += 1;
            enc_ctx.opts[new_len_end as usize].price = CMPT_INFINITY_PRICE;
        }

        let len_equal_mem = len_equal;
        let prefix_price = rep_match_price
            + cmpt_price_long_rep(enc_ctx, main_rep_index as u32, cur_state, pos_state);

        cmptlz_dp_try_cur_and_long(enc_ctx, prefix_price, cur, main_rep_index as u32, len_equal, pos_state);

        let len_equal = len_equal_mem;

        if main_rep_index == 0 {
            start_len = len_equal + 1;
        }
    }

    let mut new_longest_len = enc_ctx.longest_match_len;
    let mut match_count = enc_ctx.matches_count;

    if new_longest_len > buf_avail {
        new_longest_len = buf_avail;
        match_count = 0;
        while new_longest_len > enc_ctx.matches[match_count as usize].len {
            match_count += 1;
        }
        enc_ctx.matches[match_count as usize].len = new_longest_len;
        match_count += 1;
    }

    if new_longest_len >= start_len {
        let normal_match_prefix_price = match_price + cmpt_price_bit0(enc_ctx, enc_ctx.is_rep[cur_state as usize]);

        let mut new_len_end = len_end;
        while new_len_end < cur + new_longest_len {
            new_len_end += 1;
            enc_ctx.opts[new_len_end as usize].price = CMPT_INFINITY_PRICE;
        }

        cmptlz_dp_try_cur_and_match(
            enc_ctx,
            start_len,
            match_count,
            normal_match_prefix_price,
            cur,
            pos_state,
        );
    }

    len_end
}
