pub fn CmptRcShiftLow(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut lowLow32: u32 = rcCtx.low.cast::<u32>();
    let mut high: u64 = (rcCtx.low >> 32).cast::<u32>().cast::<u64>();
    rcCtx.low = (lowLow32 << 8).cast();
    CMPT_RC_BREAK_CHECK!(rcCtx, rcCtx.buf, res);
    if lowLow32 < 0xFF000000 || high != 0 {
        let mut buf: Ptr<u8> = rcCtx.buf.cast();
        *buf = (rcCtx.cache + high).cast::<u8>();
        buf += 1;
        rcCtx.buf = buf.cast();
        rcCtx.cache = (lowLow32 >> 24).cast::<u8>();
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
        high += 0xFF;
        loop {
            let mut buf1: Ptr<u8> = rcCtx.buf.cast();
            CMPT_RC_BREAK_SHIFTING!(rcCtx, buf1, res);
            *buf1 = high.cast::<u8>();
            buf1 += 1;
            rcCtx.buf = buf1.cast();
            rcCtx.cacheSize -= 1;
        }
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
    } else {
        rcCtx.cacheSize += 1;
    }
    return res.cast();
}
