pub fn cmpt_rc_pos_slot_process(enc_ctx: &mut CmptLzEncCtx, pos_slot: u32, len: u32) -> i32 {
    let mut shift_res = CMPT_OK;
    let mut range = enc_ctx.rc_ctx.range;
    let mut sym = pos_slot + (1 << 6);
    let mut bit0_prob;
    let mut new_bound;
    let mut bit;
    let probs = &mut enc_ctx.prob_dist_slot[get_len_to_pos_state(len)];
    
    while sym < (1 << (CMPTLZ_DIST_SLOT_BITS * 2)) {
        let pos_slot_prob_table_index = &mut probs[(sym >> CMPTLZ_DIST_SLOT_BITS) as usize];
        bit = (sym >> (CMPTLZ_DIST_SLOT_BITS - 1)) & 1;
        sym <<= 1;
        cmpt_rc_bit_process(
            enc_ctx.rc_ctx,
            pos_slot_prob_table_index,
            bit,
            &mut bit0_prob,
            &mut range,
            &mut new_bound,
            &mut shift_res,
        );
        if shift_res != CMPT_OK {
            return shift_res;
        }
    }
    
    enc_ctx.rc_ctx.range = range;
    CMPT_OK
}
