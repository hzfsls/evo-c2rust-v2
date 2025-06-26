pub fn CmptLzDecode(mut pDecIn: Ptr<CmptLzDecIn>, mut pDecOut: Ptr<CmptLzDecOut>, mut protData: Ptr<u8>, mut finMode: EnCmptLzFinMode, mut finStatus: Ptr<EnCmptLzStatus>, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    let mut inSize: usize = pDecIn.strInLen;
    let mut decProt: CmptLzDecProt = Default::default();
    let mut decCtx: CmptLzDecCtx = Default::default();
    decCtx.numProbs = 0;

    if (inSize < CMPTLZ_PROPS_SIZE!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    CmptLzDecConstruct(c_ref!(decCtx));
    res = CmptLzPropsDecode(protData, CMPTLZ_PROPS_SIZE!(), c_ref!(decProt));
    if (res != CMPT_OK!()) {
        return res;
    }
    res = CmptLzDecAllocateProbs(c_ref!(decCtx), c_ref!(decProt), memHook);
    if (res != CMPT_OK!()) {
        return res;
    }

    decCtx.prop = decProt;
    decCtx.dict = pDecOut.pDestOut;
    decCtx.dictBufSize = pDecOut.destOutLen;
    CmptLzDecInit(c_ref!(decCtx));

    *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
    res = CmptLzDecDecodeToDic(c_ref!(decCtx), pDecOut.destOutLen, pDecIn.pSrcIn, c_ref!(inSize), finMode, finStatus);
    pDecIn.strInCostLen = inSize;
    pDecOut.destOutFillLen = decCtx.dictPos;
    CmptLzDecFreeProbs(c_ref!(decCtx), memHook);

    return res;
}
