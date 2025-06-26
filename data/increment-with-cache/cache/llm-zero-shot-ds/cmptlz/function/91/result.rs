use std::ptr;

static CMPTLZ_POS_SLOT_BITS: usize = 6;
static CMPTLZ_LOW_POSSLOT: u32 = 4;
static CMPTLZ_HIGH_POSSLOT: u32 = 14;
static CMPTLZ_REP4: u32 = 4;
static CMPTLZ_LARGE_DIST_LOW_BITS: u32 = 3;
static CMPTLZ_PROB_LG_BIT: u32 = 8;

#[inline]
fn CMPTLZ_POSSLOT_BIT_DEC(
    prob: *mut CmptLzDecProb,
    range: &mut u32,
    range_code: &mut u32,
    range_bound: &mut u32,
    pos_slot: &mut u32,
    buf_to_dec: &mut *const u8,
) {
    // Implementation of the macro
    unimplemented!()
}

#[inline]
fn CMPTLZ_IS_THE_BIT_0(
    prob: *mut CmptLzDecProb,
    range: &mut u32,
    range_code: &mut u32,
    range_bound: &mut u32,
) -> bool {
    // Implementation of the macro
    unimplemented!()
}

#[inline]
fn CMPTLZ_RANGE_UPDATE_0(
    prob: *mut CmptLzDecProb,
    range: &mut u32,
    range_bound: &mut u32,
) {
    // Implementation of the macro
    unimplemented!()
}

#[inline]
fn CMPTLZ_RANGE_UPDATE_1(
    prob: *mut CmptLzDecProb,
    range: &mut u32,
    range_code: &mut u32,
    range_bound: &mut u32,
) {
    // Implementation of the macro
    unimplemented!()
}

#[inline]
fn CMPTLZ_RANGE_NORMALIZE(
    range: &mut u32,
    range_code: &mut u32,
    buf_to_dec: &mut *const u8,
) {
    // Implementation of the macro
    unimplemented!()
}

#[inline]
fn CMPTLZ_DIST_BIT_DEC(
    prob_dist: &mut *mut CmptLzDecProb,
    prob_pos_slot: *mut CmptLzDecProb,
    range: &mut u32,
    range_code: &mut u32,
    range_bound: &mut u32,
    assist_bits: &mut u32,
    cycle_sym: &mut u32,
) {
    // Implementation of the macro
    unimplemented!()
}

pub fn cmpt_lz_dist_dec(
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
    let mut buf_to_dec = dec_ctx.buf;
    let mut dist_dec;

    let prob_pos_slot = CmptLzGetPosSlotProb(probs_matrix) + CmptLzGetLenCondition(dec_len);

    for i in 0..CMPTLZ_POS_SLOT_BITS {
        CMPTLZ_POSSLOT_BIT_DEC(
            prob_pos_slot + pos_slot,
            &mut range,
            &mut range_code,
            &mut range_bound,
            &mut pos_slot,
            &mut buf_to_dec,
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
    dist_dec = CmptLzGetBaseDistByPosSlot(pos_slot);
    if pos_slot < CMPTLZ_HIGH_POSSLOT {
        assist_bits = 1;
        dist_dec <<= direct_bit_num;

        dist_dec += assist_bits;
        let mut prob_pos_slot = CmptLzGetSpecPosProb(probs_matrix);
        loop {
            if CMPTLZ_IS_THE_BIT_0(
                prob_pos_slot + dist_dec,
                &mut range,
                &mut range_code,
                &mut range_bound,
            ) {
                CMPTLZ_RANGE_UPDATE_0(
                    prob_pos_slot + dist_dec,
                    &mut range,
                    &mut range_bound,
                );
                CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
                dist_dec += assist_bits;
                assist_bits <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_1(
                    prob_pos_slot + dist_dec,
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                );
                CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
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
            CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
            range >>= 1;
            range_code -= range;
            assist_bits = (0 - ((range_code as i32) >> 31)) as u32;
            dist_dec = (dist_dec << 1) + (assist_bits + 1);
            range_code += range & assist_bits;
            direct_bit_num -= 1;
            if direct_bit_num == 0 {
                break;
            }
        }

        let mut prob_dist;
        prob_pos_slot = CmptLzGetAilgnProb(probs_matrix);

        dist_dec <<= CMPTLZ_LARGE_DIST_LOW_BITS;
        assist_bits = 1;

        let mut cycle_sym = 1;
        for _ in 0..3 {
            CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
            CMPTLZ_DIST_BIT_DEC(
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
        CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
        prob_dist = prob_pos_slot + assist_bits;
        range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_dist);
        if range_code < range_bound {
            CMPTLZ_RANGE_UPDATE_0(prob_dist, &mut range, &mut range_bound);
            assist_bits -= 8;
        } else {
            CMPTLZ_RANGE_UPDATE_1(prob_dist, &mut range, &mut range_code, &mut range_bound);
        }
        CMPTLZ_RANGE_NORMALIZE(&mut range, &mut range_code, &mut buf_to_dec);
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
