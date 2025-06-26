pub fn CmptLzDecCarefulProcess(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut bufLimit: Ptr<u8>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut remainLen: u32;
    let mut bufLimitTmp: Ptr<u8>;
    let mut pSrcIn: Ptr<u8>;
    c_do!({
        bufLimitTmp = bufLimit.cast();
        pSrcIn = decCtx.buf.cast();
        res = CmptLzTryDecOnePacket(decCtx.cast(), pSrcIn.cast(), c_ref!(bufLimitTmp).cast()).cast();
        if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
            break;
        }
        res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimitTmp.cast()).cast();
        if (res != CMPT_OK!()).as_bool() || (decCtx.buf != bufLimitTmp).as_bool() {
            return CMPT_ERROR_DATA!();
        }
        if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()).as_bool() {
            break;
        }
    } while (decCtx.dictPos < dicPosLimit);
    if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() && (decCtx.buf < bufLimit).as_bool() {
        remainLen = (bufLimit - decCtx.buf).cast();
        decCtx.tempBufSize = remainLen.cast();
        c_for!(let mut idx: u32 = 0; idx < remainLen; idx.suffix_plus_plus(); {
            decCtx.tempBuf[idx] = decCtx.buf[idx].cast();
        });
    }
    return CMPT_OK!();
}
