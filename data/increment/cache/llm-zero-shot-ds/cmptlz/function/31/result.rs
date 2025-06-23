pub fn cmpt_rc_flush_data(rc_ctx: &mut CmptRcCtx) -> i32 {
    let mut res = CMPT_OK;
    for _ in 0..5 {
        res = cmpt_rc_shift_low(rc_ctx);
        if res != CMPT_OK {
            break;
        }
    }
    res
}
