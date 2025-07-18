pub fn CmptLzDecSinglePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut psrcCostLen: Ptr<usize>) -> i32 {
    let mut res: i32;
    let mut lookAheadLen: usize = 0;
    let mut newTempBufSize: u32 = decCtx.tempBufSize;
    let mut oldTmpBuf: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + decCtx.tempBufSize;
    while newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX!() && lookAheadLen < srcInLen {
        decCtx.tempBuf[newTempBufSize] = pSrcIn[lookAheadLen];
        newTempBufSize += 1;
        lookAheadLen += 1;
    }
    let mut bufLimit: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, decCtx.tempBuf.cast(), c_ref!(bufLimit));
    if res == CMPTLZ_DEC_INPUT_EOF!() {
        *psrcCostLen = lookAheadLen;
        decCtx.tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF!();
    }
    if res == CMPT_ERROR_DATA!() {
        return res;
    }
    decCtx.buf = decCtx.tempBuf.cast();
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if res != CMPT_OK!() || bufLimit != decCtx.buf || bufLimit <= oldTmpBuf {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA!();
    }
    *psrcCostLen = (bufLimit - oldTmpBuf).cast();
    decCtx.tempBufSize = 0;
    return res;
}