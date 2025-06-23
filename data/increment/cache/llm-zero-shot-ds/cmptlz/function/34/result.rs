pub fn cmpt_rc_pos_slot_process(enc_ctx: &mut CmptLzEncCtx, pos_slot: u32, len: u32) -> Result<(), CmptError> {
    let mut range = enc_ctx.rc_ctx.range;
    let mut sym = pos_slot + (1 << 6);
    let probs = enc_ctx.prob_dist_slot[get_len_to_pos_state(len)];
    
    while sym < (1 << (CMPTLZ_DIST_SLOT_BITS * 2)) {
        let pos_slot_prob_table_index = &mut probs[(sym >> CMPTLZ_DIST_SLOT_BITS) as usize];
        let bit = (sym >> (CMPTLZ_DIST_SLOT_BITS - 1)) & 1;
        sym <<= 1;
        
        let (new_range, shift_res) = cmpt_rc_bit_process(
            &mut enc_ctx.rc_ctx,
            pos_slot_prob_table_index,
            bit,
            range,
        )?;
        range = new_range;
    }
    
    enc_ctx.rc_ctx.range = range;
    Ok(())
}
