pub fn CmptRcShiftLow(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut lowLow32: u32 = rcCtx.low.cast();
    let mut high: u32 = (rcCtx.low >> 32).cast();
    rcCtx.low = (lowLow32 << 8);
    CMPT_RC_BREAK_CHECK!(rcCtx, rcCtx.buf, res);
    if (lowLow32 < 0xFF000000 || high != 0) {
        let mut buf: Ptr<u8> = rcCtx.buf;
        *buf = (rcCtx.cache + high).cast();
        buf += 1;
        rcCtx.buf = buf;
        rcCtx.cache = (lowLow32 >> 24).cast();
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
        high += 0xFF;
        loop {
            let mut buf1: Ptr<u8> = rcCtx.buf;
            CMPT_RC_BREAK_SHIFTING!(rcCtx, buf1, res);
            *buf1 = high.cast();
            buf1 += 1;
            rcCtx.buf = buf1;
            rcCtx.cacheSize -= 1;
        }
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
    } else {
        rcCtx.cacheSize += 1;
    }
    return res;
}