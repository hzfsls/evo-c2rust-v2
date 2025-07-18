pub fn CmptlzFlush(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    encCtx.encNeedFinish = true;
    if encCtx.endMarker != 0 {
        CmptlzEndMarker();
    }
    CmptRcFlushData(encCtx.rcCtx);
    return CmptRcFlush64Kb(encCtx.rcCtx);
}