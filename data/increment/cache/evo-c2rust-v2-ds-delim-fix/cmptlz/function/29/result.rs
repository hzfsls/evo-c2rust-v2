pub fn CmptRcFlush64Kb(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut flushOutLen: usize = (rcCtx.buf - rcCtx.bufBase).cast();
    let mut res: i32 = c_memcpy_s!(rcCtx.outBuf, rcCtx.outBufLeft, rcCtx.bufBase, flushOutLen);
    if (res != 0).as_bool() {
        return CMPT_ENC_ERROR_WRITE!();
    }
    rcCtx.outBuf += flushOutLen;
    rcCtx.outBufLeft -= flushOutLen;
    rcCtx.buf = rcCtx.bufBase.cast();
    return CMPT_OK!();
}
