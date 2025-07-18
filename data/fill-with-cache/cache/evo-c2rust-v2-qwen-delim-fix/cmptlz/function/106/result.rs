pub fn CmptRcPrepare(mut encCtx: Ptr<CmptLzEncCtx>, mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = alloc.CmptLzAlloc(CMPTLZ_RC_CCTX_HANDLE!(), c_sizeof!(CmptRcCtx)).cast();
    if (rc == NULL!()).as_bool() {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    c_memset_s!(rc.cast::<Ptr<Void>>(), c_sizeof!(CmptRcCtx), 0, c_sizeof!(CmptRcCtx)).cast::<Void>();
    encCtx.rcCtx = rc.cast();
    rc.bufBase = alloc.CmptLzAlloc(CMPTLZ_RC_BUF_HANDLE!(), CMPTLZ_RC_BUFFER_SIZE!()).cast();
    c_memset_s!(rc.bufBase.cast::<Ptr<Void>>(), CMPTLZ_RC_BUFFER_SIZE!(), 0, CMPTLZ_RC_BUFFER_SIZE!()).cast::<Void>();
    if (rc.bufBase == NULL!()).as_bool() {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    rc.outBufLeft = (*destLen).cast();
    rc.outBuf = dest.cast();
    rc.buf = rc.bufBase.cast();
    rc.range = 0xFFFFFFFF;
    rc.cacheSize = 0;
    rc.cache = 0;
    rc.low = 0;
    return 0;
}