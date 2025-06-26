use std::ptr;

static CMPT_ERROR_DATA: i32 = -1;
static CMPT_OK: i32 = 0;
static CMPTLZ_LIT_STATES: u32 = 7;

#[inline]
fn cmpt_lz_try_dec_lit_packet(
    dec_ctx: &mut CmptLzDecCtx,
    range: u32,
    range_code: u32,
    range_bound: u32,
    buf_try_dec: *const u8,
    pbuf_limit: *mut *const u8,
) -> i32 {
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);
    let proc_pos = dec_ctx.processed_pos;
    let lit_pos_mask = ((0x100 << dec_ctx.prop.lit_pos) - (0x100 >> dec_ctx.prop.lit_ctx)) as u32;

    let dict_buf_size = dec_ctx.dict_buf_size;
    let dic_pos = dec_ctx.dict_pos;
    let dict = dec_ctx.dict.as_ptr();
    let mut buf_limit = unsafe { *pbuf_limit };

    if dec_ctx.dict_pos >= dec_ctx.dict_buf_size {
        return CMPT_ERROR_DATA;
    }

    let mut prob_slot = cmpt_lz_get_literal_prob(probs_matrix);
    if proc_pos != 0 || dec_ctx.check_dic_size != 0 {
        let prev_pos = if dic_pos == 0 { dict_buf_size } else { dic_pos } - 1;
        let prev_byte = unsafe { *dict.offset(prev_pos as isize) };
        prob_slot = prob_slot.offset(
            (3 * ((((proc_pos << 8) + prev_byte as u32) & lit_pos_mask) << dec_ctx.prop.lit_ctx)) as isize
        );
    }

    let mut dec_sym = 1;
    if dec_ctx.state < CMPTLZ_LIT_STATES {
        while dec_sym < 0x100 {
            let prob_bit = unsafe { prob_slot.offset(dec_sym as isize) };
            cmptlz_single_bit_try_dec(
                &mut range,
                &mut range_code,
                range_bound,
                &mut dec_sym,
                prob_bit,
            );
            cmptlz_range_try_normalize(
                &mut range,
                &mut range_code,
                &mut buf_try_dec,
                &mut buf_limit,
            );
        }
    } else {
        let match_pos = dic_pos - dec_ctx.reps[0] + if dic_pos < dec_ctx.reps[0] { dict_buf_size } else { 0 };
        let mut match_sym = unsafe { *dict.offset(match_pos as isize) } as u32;
        let mut offset = 0x100;
        while dec_sym < 0x100 {
            match_sym <<= 1;
            let bit = offset;
            offset &= match_sym;
            let prob_bit = unsafe { prob_slot.offset((offset + bit + dec_sym) as isize) };
            cmptlz_match_bit_try_dec(
                &mut range,
                &mut range_code,
                range_bound,
                &mut dec_sym,
                prob_bit,
            );
            cmptlz_range_try_normalize(
                &mut range,
                &mut range_code,
                &mut buf_try_dec,
                &mut buf_limit,
            );
        }
    }

    cmptlz_range_try_normalize(
        &mut range,
        &mut range_code,
        &mut buf_try_dec,
        &mut buf_limit,
    );
    unsafe { *pbuf_limit = buf_try_dec };
    CMPT_OK
}
