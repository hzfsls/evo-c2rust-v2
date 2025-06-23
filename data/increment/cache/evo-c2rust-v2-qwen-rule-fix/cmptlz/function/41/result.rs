pub fn CmptLzDecCtxPrepare(mut decCtx: Ptr<CmptLzDecCtx>, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut finStatus: Ptr<EnCmptLzStatus>) -> i32 {
    let mut readCodeLen: usize = (CMPTLZ_RANGE_CODE_SIZE!() - decCtx.tempBufSize).cast();
    readCodeLen = if srcInLen < readCodeLen { srcInLen } else { readCodeLen };
    while readCodeLen > 0 {
        let tmp0 = decCtx.tempBufSize;
        decCtx.tempBuf[tmp0] = *pSrcIn;
        decCtx.tempBufSize += 1;
        pSrcIn += 1;
        readCodeLen -= 1;
    }
    if (decCtx.tempBufSize != 0) && (decCtx.tempBuf[0] != 0) {
        decCtx.tempBufSize = 0;
        *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
        return CMPT_ERROR_DATA!();
    }
    if (decCtx.tempBufSize < CMPTLZ_RANGE_CODE_SIZE!()) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    CmptLzRangeCodeInit(decCtx);
    if (decCtx.remainLen > (CMPTLZ_MATCH_MAX_LEN!() + 1)) {
        CmptLzDecGetProbsInit(decCtx);
        decCtx.reps[0] = 1;
        decCtx.reps[1] = 1;
        decCtx.reps[2] = 1;
        decCtx.reps[3] = 1;
    }
    decCtx.remainLen = 0;
    return CMPT_OK!();
}