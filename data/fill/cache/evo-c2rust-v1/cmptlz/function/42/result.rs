pub fn CmptLzDecSinglePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut psrcCostLen: Ptr<usize>) -> i32 {
    let mut res: i32;
    let mut lookAheadLen: usize = 0;
    let mut newTempBufSize: u32 = decCtx.tempBufSize.cast();
    let mut oldTmpBuf: Ptr<u8> = (c_ref!(decCtx.tempBuf[0]) + decCtx.tempBufSize).cast();
    while newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX!() && lookAheadLen < srcInLen {
        decCtx.tempBuf[newTempBufSize] = pSrcIn[lookAheadLen].cast();
        newTempBufSize += 1;
        lookAheadLen += 1;
    }
    let mut bufLimit: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx.cast(), decCtx.tempBuf.cast(), c_ref!(bufLimit).cast()).cast();
    if res == CMPTLZ_DEC_INPUT_EOF!() {
        *psrcCostLen = lookAheadLen.cast();
        decCtx.tempBufSize = newTempBufSize.cast();
        return CMPTLZ_DEC_INPUT_EOF!();
    }
    if res == CMPT_ERROR_DATA!() {
        return res;
    }
    decCtx.buf = c_ref!(decCtx.tempBuf[0]).cast();
    res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimit.cast()).cast();
    if res != CMPT_OK!() || bufLimit != decCtx.buf || bufLimit <= oldTmpBuf {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA!();
    }
    *psrcCostLen = (bufLimit - oldTmpBuf).cast::<usize>();
    decCtx.tempBufSize = 0;
    return res.cast();
}
