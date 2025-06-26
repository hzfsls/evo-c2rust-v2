use std::os::raw::c_int;

// Assuming CmptRcCtx, CmptlzProb, CMPT_OK, CMPT_RC_BIT_PROCESS, CMPTLZ_RETURN_IF_NOT_OK are defined elsewhere
// Also assuming uint32_t is u32

pub unsafe fn cmpt_rc_lit_process(rc_ctx: *mut CmptRcCtx, prob: *mut CmptlzProb, sym: u32) -> c_int {
    let mut shift_res = CMPT_OK;
    let mut range = (*rc_ctx).range;
    let mut bit0_prob;
    let mut new_bound;
    let mut cur_bit;

    let mut sym = sym | 0x100;
    while sym < 0x10000 {
        let lit_prob_table_index = prob.offset((sym >> 8) as isize);
        cur_bit = (sym >> 7) & 1;
        
        CMPT_RC_BIT_PROCESS(rc_ctx, lit_prob_table_index, cur_bit, bit0_prob, range, new_bound, shift_res);
        if shift_res != CMPT_OK {
            return shift_res;
        }
        
        sym <<= 1;
    }
    
    (*rc_ctx).range = range;
    CMPT_OK
}
