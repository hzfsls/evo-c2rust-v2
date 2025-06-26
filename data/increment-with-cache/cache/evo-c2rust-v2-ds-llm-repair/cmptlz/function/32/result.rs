pub fn CmptRcCtxInit(mut rcCtx: Ptr<CmptRcCtx>) {
    rcCtx.range = 0xFFFFFFFF;
    rcCtx.cache = 0;
    rcCtx.low = 0;
    rcCtx.cacheSize = 0;
    rcCtx.buf = rcCtx.bufBase.cast();
}
