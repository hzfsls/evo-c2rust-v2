pub fn CmptLzDecCarefulProcess(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut bufLimit: Ptr<u8>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut remainLen: u32;
    let mut bufLimitTmp: Ptr<u8>;
    let mut pSrcIn: Ptr<u8>;
    loop {
        bufLimitTmp = bufLimit;
        pSrcIn = decCtx.buf;
        res = CmptLzTryDecOnePacket(decCtx, pSrcIn, c_ref!(bufLimitTmp));
        if (res == CMPTLZ_DEC_INPUT_EOF!()) {
            break;
        }
        res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimitTmp);
        if (res != CMPT_OK!()) || (decCtx.buf != bufLimitTmp) {
            return CMPT_ERROR_DATA!();
        }
        if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()) {
            break;
        }
        if (decCtx.dictPos < dicPosLimit) {
            continue;
        } else {
            break;
        }
    }
    if (res == CMPTLZ_DEC_INPUT_EOF!()) && (decCtx.buf < bufLimit) {
        remainLen = (bufLimit - decCtx.buf).cast::<u32>();
        decCtx.tempBufSize = remainLen.cast();
        c_for!(let mut idx: u32 = 0; idx < remainLen; idx.suffix_plus_plus(); {
            let tmp0 = idx;
            decCtx.tempBuf[tmp0];
        });
    }
    return CMPT_OK!();
}