pub fn CmptLzDecDecodeToDic(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut pStrInLen: Ptr<usize>, mut finMode: EnCmptLzFinMode, mut finStatus: Ptr<EnCmptLzStatus>) -> i32 {
    let mut res: i32;
    let mut carefulDecDone: bool = false;
    let mut srcDecLenTmp: usize;
    let mut srcDecLen: usize = 0;
    let mut srcInLen: usize = *pStrInLen;

    if (decCtx.remainLen > CMPTLZ_MATCH_MAX_LEN!()).as_bool() {
        let mut oldTempBufSize: usize = decCtx.tempBufSize;
        res = CmptLzDecCtxPrepare(decCtx.cast(), pSrcIn.cast(), srcInLen.cast(), finStatus.cast()).cast();
        srcDecLenTmp = (decCtx.tempBufSize - oldTempBufSize);
        if (res != CMPT_OK!()).as_bool() || (*finStatus == CMPTLZ_STATUS_NEEDS_MORE_INPUT!()).as_bool() {
            *pStrInLen = srcDecLenTmp.cast();
            return res;
        }
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        decCtx.tempBufSize = 0;
    }

    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()).as_bool() {
        if (decCtx.code != 0).as_bool() {
            return CMPT_ERROR_DATA!();
        }
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }

    if (decCtx.remainLen != 0).as_bool() {
        CmptLzDecRemWriteInDict(decCtx.cast(), dicPosLimit.cast());
    }

    if (decCtx.tempBufSize != 0).as_bool() {
        res = CmptLzDecSinglePacket(decCtx.cast(), dicPosLimit.cast(), pSrcIn.cast(), srcInLen.cast(), c_ref!(srcDecLenTmp).cast());
        *pStrInLen = srcDecLenTmp.cast();
        if (res == CMPT_ERROR_DATA!()).as_bool() {
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
            return CMPT_ERROR_DATA!();
        } else if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
            *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
            return CMPT_OK!();
        } else {
            srcDecLen += srcDecLenTmp;
            pSrcIn += srcDecLenTmp;
            srcInLen -= srcDecLenTmp;
        }
    }

    while (decCtx.dictPos < dicPosLimit).as_bool() && (carefulDecDone == false).as_bool() {
        decCtx.buf = pSrcIn.cast();
        if (srcInLen <= CMPTLZ_REQUIRED_INPUT_MAX!()).as_bool() {
            res = CmptLzDecCarefulProcess(decCtx.cast(), dicPosLimit.cast(), (pSrcIn + srcInLen).cast());
            carefulDecDone = true;
        } else {
            res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), (pSrcIn + srcInLen - CMPTLZ_REQUIRED_INPUT_MAX!()).cast());
        }
        srcDecLenTmp = (decCtx.buf - pSrcIn).cast::<usize>() + decCtx.tempBufSize;
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;

        if (res == CMPT_ERROR_DATA!()).as_bool() {
            *pStrInLen = srcDecLen.cast();
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
            return CMPT_ERROR_DATA!();
        }
    }

    *pStrInLen = srcDecLen.cast();
    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()).as_bool() && (decCtx.code == 0).as_bool() {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }
    if (decCtx.dictPos < dicPosLimit).as_bool() {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen == 0).as_bool() && (decCtx.code == 0).as_bool() {
        *finStatus = CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK!();
        return CMPT_OK!();
    }
    if (finMode == CMPTLZ_FINISH_ANY!()).as_bool() {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen != 0).as_bool() {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
        return CMPT_ERROR_DATA!();
    }

    srcDecLenTmp = 0;
    res = CmptLzDecSinglePacket(decCtx.cast(), dicPosLimit.cast(), pSrcIn.cast(), srcInLen.cast(), c_ref!(srcDecLenTmp).cast());
    srcDecLen += srcDecLenTmp;
    *pStrInLen = srcDecLen.cast();
    if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()).as_bool() && (decCtx.code == 0).as_bool() {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }

    *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
    return CMPT_ERROR_DATA!();
}
