pub fn cmpt_rc_shift_low(rc_ctx: &mut CmptRcCtx) -> CmptResult {
    let mut res = CmptResult::Ok;
    let low_low32 = rc_ctx.low as u32;
    let high = (rc_ctx.low >> 32) as u32;
    rc_ctx.low = (low_low32 << 8) as u64;
    
    // CMPT_RC_BREAK_CHECK equivalent
    if rc_ctx.buf.is_null() {
        return CmptResult::Break;
    }
    
    if low_low32 < 0xFF000000 || high != 0 {
        unsafe {
            let buf = rc_ctx.buf;
            *buf = (rc_ctx.cache + high) as u8;
            rc_ctx.buf = buf.add(1);
            rc_ctx.cache = (low_low32 >> 24) as u8;
            
            // CMPT_RC_BREAK_SHIFTING equivalent
            if rc_ctx.buf.is_null() {
                return CmptResult::Break;
            }
            
            let mut high = high + 0xFF;
            loop {
                let buf1 = rc_ctx.buf;
                
                // CMPT_RC_BREAK_SHIFTING equivalent
                if buf1.is_null() {
                    return CmptResult::Break;
                }
                
                *buf1 = high as u8;
                rc_ctx.buf = buf1.add(1);
                rc_ctx.cache_size -= 1;
            }
        }
    } else {
        rc_ctx.cache_size += 1;
    }
    
    res
}
