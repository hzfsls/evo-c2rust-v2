pub struct CmptRcCtx {
    range: u32,
    cache: u32,
    low: u32,
    cacheSize: u32,
    buf: *mut u8,
    bufBase: *mut u8,
}

pub fn cmpt_rc_ctx_init(rc_ctx: &mut CmptRcCtx) {
    rc_ctx.range = 0xFFFFFFFF;
    rc_ctx.cache = 0;
    rc_ctx.low = 0;
    rc_ctx.cacheSize = 0;
    rc_ctx.buf = rc_ctx.bufBase;
}
