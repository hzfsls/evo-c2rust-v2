use std::cmp::{min, max};

const CMPTLZ_UINT32_MAX: u32 = u32::MAX;
const CMPTLZ_MATCH_LEN_MIN: u32 = 2;
const CMPTLZ_NUM_REPS: usize = 4;
const CMPT_INFINITY_PRICE: u32 = u32::MAX;

#[inline]
fn not_equal_2_bytes(a: &[u8], b: &[u8]) -> bool {
    a[0] != b[0] || a[1] != b[1]
}

fn cmptlz_dp_init(
    enc_ctx: &mut CmptLzEncCtx,
    mf: &mut CmptMfCtx,
    position: u32,
) -> u32 {
    let nice_len = mf.nice_len;
    let len_main;
    let matches_count;

    if mf.read_ahead == 0 {
        len_main = cmptlz_match_finder(mf, &mut matches_count, &mut enc_ctx.matches);
    } else {
        len_main = enc_ctx.longest_match_len;
        matches_count = enc_ctx.matches_count;
    }

    let buf = cmpt_mf_get_ptr(mf).sub(1);
    let buf_avail = min(cmpt_mf_avail(mf) + 1, CMPT_MF_LONGEST_MATCH);

    if buf_avail < CMPTLZ_MATCH_LEN_MIN {
        enc_ctx.back_res = CMPTLZ_UINT32_MAX;
        enc_ctx.len_res = 1;
        return CMPTLZ_UINT32_MAX;
    }

    let mut rep_lens = [0; CMPTLZ_NUM_REPS];
    let mut rep_max_index = 0;

    for i in 0..CMPTLZ_NUM_REPS {
        let buf_back = buf.sub(enc_ctx.reps[i] + 1);

        if not_equal_2_bytes(buf, buf_back) {
            rep_lens[i] = 0;
            continue;
        }
        rep_lens[i] = cmpt_mem_cmp_len_safe(buf, buf_back, CMPTLZ_MATCH_LEN_MIN, buf_avail);
        if rep_lens[i] > rep_lens[rep_max_index] {
            rep_max_index = i;
        }
    }

    if rep_lens[rep_max_index] >= nice_len {
        enc_ctx.back_res = rep_max_index as u32;
        enc_ctx.len_res = rep_lens[rep_max_index];
        cmptlz_match_skiper(mf, rep_lens[rep_max_index] - 1);
        return CMPTLZ_UINT32_MAX;
    }

    if len_main >= nice_len {
        enc_ctx.back_res = enc_ctx.matches[matches_count - 1].dist + CMPTLZ_NUM_REPS as u32;
        enc_ctx.len_res = len_main;
        cmptlz_match_skiper(mf, len_main - 1);
        return CMPTLZ_UINT32_MAX;
    }

    let current_byte = *buf;
    let match_byte = *(buf.sub(enc_ctx.reps[0] + 1));
    let len_end = max(len_main, rep_lens[rep_max_index]);
    if len_end < CMPTLZ_MATCH_LEN_MIN && current_byte != match_byte {
        enc_ctx.back_res = CMPTLZ_UINT32_MAX;
        enc_ctx.len_res = 1;
        return CMPTLZ_UINT32_MAX;
    }

    enc_ctx.opts[0].state = enc_ctx.state;

    let pos_state = position & enc_ctx.pos_mask;

    enc_ctx.lit_marcov.pos = position;
    enc_ctx.lit_marcov.prev_byte = *(buf.sub(1));
    let is_literal_state = enc_ctx.state < 7;
    let is_match_mode = !is_literal_state;

    enc_ctx.opts[1].price = cmpt_price_bit0(enc_ctx, enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize]) +
                            cmpt_price_literal(enc_ctx, is_match_mode, match_byte, current_byte);
    enc_ctx.opts[1].back_prev = CMPTLZ_UINT32_MAX;

    let match_price = cmpt_price_bit1(enc_ctx, enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize]);
    let rep_match_price = match_price + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep[enc_ctx.state as usize]);

    if match_byte == current_byte {
        cmptlz_dp_init_short_rep(enc_ctx, rep_match_price, pos_state);
    }

    if len_end < CMPTLZ_MATCH_LEN_MIN {
        enc_ctx.back_res = enc_ctx.opts[1].back_prev;
        enc_ctx.len_res = 1;
        return CMPTLZ_UINT32_MAX;
    }

    enc_ctx.opts[1].pos_prev = 0;
    for i in 0..CMPTLZ_NUM_REPS {
        enc_ctx.opts[0].backs[i] = enc_ctx.reps[i];
    }

    let mut len = len_end;
    while len >= CMPTLZ_MATCH_LEN_MIN {
        enc_ctx.opts[len as usize].price = CMPT_INFINITY_PRICE;
        len -= 1;
    }

    cmptlz_dp_init_long_rep(enc_ctx, &rep_lens, rep_match_price, pos_state);

    let normal_match_price = match_price + cmpt_price_bit0(enc_ctx, enc_ctx.is_rep[enc_ctx.state as usize]);
    let mut len = if rep_lens[0] > CMPTLZ_MATCH_LEN_MIN {
        rep_lens[0] + 1
    } else {
        CMPTLZ_MATCH_LEN_MIN
    };

    if len <= len_main {
        cmptlz_dp_init_match(enc_ctx, matches_count, normal_match_price, pos_state, len);
    }
    len_end
}
