pub fn CmptRcPrepare(mut encCtx: Ptr<CmptLzEncCtx>, mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = (alloc.CmptLzAlloc)(CMPTLZ_RC_CCTX_HANDLE!(), c_sizeof!(CmptRcCtx).cast::<usize>()).cast::<Ptr<CmptRcCtx>>();
    if (rc == NULL!()) {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    c_memset_s!(rc, c_sizeof!(CmptRcCtx), 0, c_sizeof!(CmptRcCtx)).cast::<Void>();
    encCtx.rcCtx = rc;
    rc.bufBase = (alloc.CmptLzAlloc)(CMPTLZ_RC_BUF_HANDLE!(), CMPTLZ_RC_BUFFER_SIZE!().cast::<usize>()).cast::<Ptr<u8>>();
    c_memset_s!(rc.bufBase, CMPTLZ_RC_BUFFER_SIZE!(), 0, CMPTLZ_RC_BUFFER_SIZE!()).cast::<Void>();
    if (rc.bufBase == NULL!()) {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    rc.outBufLeft = *destLen;
    rc.outBuf = dest;
    rc.buf = rc.bufBase;
    rc.range = 0xFFFFFFFF;
    rc.cacheSize = 0;
    rc.cache = 0;
    rc.low = 0;
    return 0;
}
