pub fn CmptLzDecode(mut pDecIn: Ptr<CmptLzDecIn>, mut pDecOut: Ptr<CmptLzDecOut>, mut protData: Ptr<u8>, mut finMode: EnCmptLzFinMode, mut finStatus: Ptr<EnCmptLzStatus>, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    let mut inSize: usize = pDecIn.strInLen.cast();
    let mut decProt: CmptLzDecProt = Default::default();
    let mut decCtx: CmptLzDecCtx = Default::default();
    decCtx.numProbs = 0;

    if (inSize < CMPTLZ_PROPS_SIZE!()).as_bool() {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    CmptLzDecConstruct(c_ref!(decCtx).cast());
    res = CmptLzPropsDecode(protData.cast(), CMPTLZ_PROPS_SIZE!(), c_ref!(decProt).cast()).cast();
    if (res != CMPT_OK!()).as_bool() {
        return res;
    }
    res = CmptLzDecAllocateProbs(c_ref!(decCtx).cast(), c_ref!(decProt).cast(), memHook.cast()).cast();
    if (res != CMPT_OK!()).as_bool() {
        return res;
    }

    decCtx.prop = decProt.cast();
    decCtx.dict = pDecOut.pDestOut.cast();
    decCtx.dictBufSize = pDecOut.destOutLen.cast();
    CmptLzDecInit(c_ref!(decCtx).cast());

    *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
    res = CmptLzDecDecodeToDic(c_ref!(decCtx).cast(), pDecOut.destOutLen.cast(), pDecIn.pSrcIn.cast(), c_ref!(inSize).cast(), finMode.cast(), finStatus.cast()).cast();
    pDecIn.strInCostLen = inSize.cast();
    pDecOut.destOutFillLen = decCtx.dictPos.cast();
    CmptLzDecFreeProbs(c_ref!(decCtx).cast(), memHook.cast());

    return res.cast();
}
