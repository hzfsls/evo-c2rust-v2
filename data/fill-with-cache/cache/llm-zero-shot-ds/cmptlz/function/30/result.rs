use std::ptr;

static CMPTLZ_POS_SLOT_BITS: usize = 6;
static CMPTLZ_LOW_POSSLOT: u32 = 4;
static CMPTLZ_HIGH_POSSLOT: u32 = 14;
static CMPTLZ_REP4: u32 = 4;
static CMPTLZ_LARGE_DIST_LOW_BITS: u32 = 3;
static CMPTLZ_PROB_LG_BIT: u32 = 8;

fn cmpt_lz_dist_dec(
    dec_ctx: &mut CmptLzDecCtx,
    probs_matrix: &mut CmptLzDecProb,
    p_range: &mut u32,
    p_range_code: &mut u32,
    p_range_bound: &mut u32,
    dec_len: u32,
) -> usize {
    let mut assist_bits;
    let mut pos_slot = 1;
    let mut range = *p_range;
    let mut range_code = *p_range_code;
    let mut range_bound = *p_range_bound;
    let buf_to_dec = dec_ctx.buf;
    let mut dist_dec;
    let prob_pos_slot = cmpt_lz_get_pos_slot_prob(probs_matrix) + cmpt_lz_get_len_condition(dec_len);
    let mut i = 0;
    
    for i in 0..CMPTLZ_POS_SLOT_BITS {
        cmpt_lz_posslot_bit_dec(
            unsafe { prob_pos_slot.add(pos_slot as usize) },
            &mut range,
            &mut range_code,
            &mut range_bound,
            &mut pos_slot,
            buf_to_dec,
        );
    }
    
    pos_slot -= 64;
    
    if pos_slot < CMPTLZ_LOW_POSSLOT {
        dist_dec = pos_slot;
        cmpt_lz_dist_dec_helper(
            dec_ctx,
            dist_dec,
            buf_to_dec,
            p_range,
            p_range_code,
            p_range_bound,
            range,
            range_code,
            range_bound,
        );
        
        if dist_dec == 0xFFFFFFFF {
            return dist_dec as usize;
        } else {
            return (dist_dec + 1) as usize;
        }
    }
    
    let mut direct_bit_num = (pos_slot >> 1) - 1;
    dist_dec = cmpt_lz_get_base_dist_by_pos_slot(pos_slot);
    
    if pos_slot < CMPTLZ_HIGH_POSSLOT {
        assist_bits = 1;
        dist_dec <<= direct_bit_num;
        dist_dec += assist_bits;
        let prob_pos_slot = cmpt_lz_get_spec_pos_prob(probs_matrix);
        
        loop {
            if cmpt_lz_is_the_bit_0(
                unsafe { prob_pos_slot.add(dist_dec as usize) },
                &mut range,
                &mut range_code,
                &mut range_bound,
            ) {
                cmpt_lz_range_update_0(
                    unsafe { prob_pos_slot.add(dist_dec as usize) },
                    &mut range,
                    &mut range_bound,
                );
                cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
                dist_dec += assist_bits;
                assist_bits <<= 1;
            } else {
                cmpt_lz_range_update_1(
                    unsafe { prob_pos_slot.add(dist_dec as usize) },
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                );
                cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
                assist_bits <<= 1;
                dist_dec += assist_bits;
            }
            
            direct_bit_num -= 1;
            if direct_bit_num == 0 {
                break;
            }
        }
        
        dist_dec -= assist_bits;
    } else {
        direct_bit_num -= CMPTLZ_REP4;
        
        loop {
            cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
            range >>= 1;
            range_code -= range;
            assist_bits = 0u32.wrapping_sub((range_code >> 31) as u32);
            dist_dec = (dist_dec << 1) + (assist_bits + 1);
            range_code += range & assist_bits;
            
            direct_bit_num -= 1;
            if direct_bit_num == 0 {
                break;
            }
        }
        
        let prob_dist;
        let prob_pos_slot = cmpt_lz_get_ailgn_prob(probs_matrix);
        dist_dec <<= CMPTLZ_LARGE_DIST_LOW_BITS;
        assist_bits = 1;
        let mut cycle_sym = 1;
        
        for i in 0..3 {
            cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
            cmpt_lz_dist_bit_dec(
                &mut prob_dist,
                prob_pos_slot,
                &mut range,
                &mut range_code,
                &mut range_bound,
                &mut assist_bits,
                &mut cycle_sym,
            );
            cycle_sym <<= 1;
        }
        
        cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
        let prob_dist = unsafe { prob_pos_slot.add(assist_bits as usize) };
        range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_dist);
        
        if range_code < range_bound {
            cmpt_lz_range_update_0(prob_dist, &mut range, &mut range_bound);
            assist_bits -= 8;
        } else {
            cmpt_lz_range_update_1(prob_dist, &mut range, &mut range_code, &mut range_bound);
        }
        
        cmpt_lz_range_normalize(&mut range, &mut range_code, buf_to_dec);
        dist_dec |= assist_bits;
    }
    
    cmpt_lz_dist_dec_helper(
        dec_ctx,
        dist_dec,
        buf_to_dec,
        p_range,
        p_range_code,
        p_range_bound,
        range,
        range_code,
        range_bound,
    );
    
    if dist_dec == 0xFFFFFFFF {
        dist_dec as usize
    } else {
        (dist_dec + 1) as usize
    }
}
