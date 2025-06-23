pub fn CmptlzFlush(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    encCtx.encNeedFinish = true;
    if (encCtx.endMarker != 0).as_bool() {
        CmptlzEndMarker();
    }
    CmptRcFlushData(encCtx.rcCtx.cast());
    return CmptRcFlush64Kb(encCtx.rcCtx.cast()).cast();
}
