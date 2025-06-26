pub fn CmptLzDecInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    decCtx.dictPos = 0;
    decCtx.tempBufSize = 0;
    decCtx.processedPos = 0;
    decCtx.checkDicSize = 0;
    decCtx.remainLen = CMPTLZ_MATCH_MAX_LEN!() + 2;
}
