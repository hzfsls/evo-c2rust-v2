pub fn cmpt_rc_dist_process(
    enc_ctx: &mut CmptLzEncCtx,
    pos_slot: u32,
    dist: u32,
) -> Result<(), CmptError> {
    let footer_bits = (pos_slot >> 1) - 1;
    
    if dist < CMPT_DIST_LIMIT_2 {
        let base = (2 | (pos_slot & 1)) << footer_bits;
        cmpt_rc_reverse_process(
            enc_ctx.rc_ctx,
            enc_ctx.prob_dist_special + base,
            footer_bits,
            dist,
        )?;
    } else {
        let mut pos2 = (dist | 0xF) << (32 - footer_bits);
        let mut range = enc_ctx.rc_ctx.range;
        
        loop {
            range >>= 1;
            enc_ctx.rc_ctx.low += range & (0u32.wrapping_sub(pos2 >> 31));
            pos2 = pos2.wrapping_add(pos2);
            cmpt_rc_normalize(enc_ctx.rc_ctx, &mut range)?;
            
            if pos2 == 0xF000_0000 {
                break;
            }
        }
        
        let mut m = 1;
        let mut bit;
        let mut bit0_prob;
        let mut new_bound;
        
        for k in 0..CMPTLZ_ALIGN_BITS - 1 {
            bit = dist & 1;
            let dist = dist >> 1;
            cmpt_rc_bit_process(
                enc_ctx.rc_ctx,
                enc_ctx.prob_align + m,
                bit,
                &mut bit0_prob,
                &mut range,
                &mut new_bound,
            )?;
            m = (m << 1) + bit;
        }
        
        bit = dist & 1;
        cmpt_rc_bit_process(
            enc_ctx.rc_ctx,
            enc_ctx.prob_align + m,
            bit,
            &mut bit0_prob,
            &mut range,
            &mut new_bound,
        )?;
        
        enc_ctx.rc_ctx.range = range;
    }
    
    Ok(())
}
