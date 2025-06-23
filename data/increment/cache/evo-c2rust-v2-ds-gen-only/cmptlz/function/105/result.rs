pub fn CmptLzDecDecodeToBuf(mut decCtx: Ptr<CmptLzDecCtx>, mut pDecIn: Ptr<CmptLzDecIn>, mut pDecOut: Ptr<CmptLzDecOut>, mut finMode: EnCmptLzFinMode, mut finStatus: Ptr<EnCmptLzStatus>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut dictPos: usize;
    let mut dictPosLimit: usize;
    let mut srcCostSize: usize;

    let mut leftSrcSize: usize = pDecIn.strInLen.cast();
    let mut leftDestSize: usize = pDecOut.destOutLen.cast();
    let mut pSrcIn: Ptr<u8> = pDecIn.pSrcIn.cast();
    let mut pDestOut: Ptr<u8> = pDecOut.pDestOut.cast();
    let mut tmpFinMode: EnCmptLzFinMode;

    c_do!({
        if (decCtx.dictPos == decCtx.dictBufSize).as_bool() {
            decCtx.dictPos = 0;
        }
        dictPos = decCtx.dictPos.cast();
        if (leftDestSize > decCtx.dictBufSize - dictPos).as_bool() {
            dictPosLimit = decCtx.dictBufSize.cast();
            tmpFinMode = CMPTLZ_FINISH_ANY!();
        } else {
            dictPosLimit = (dictPos + leftDestSize).cast();
            tmpFinMode = finMode.cast();
        }
        srcCostSize = leftSrcSize.cast();
        res = CmptLzDecDecodeToDic(decCtx.cast(), dictPosLimit.cast(), pSrcIn.cast(), c_ref!(srcCostSize).cast(), tmpFinMode.cast(), finStatus.cast()).cast();
        if (res != CMPT_OK!()).as_bool() {
            break;
        }
        pSrcIn += srcCostSize;
        leftSrcSize -= srcCostSize;
        dictPosLimit = (decCtx.dictPos - dictPos).cast();
        leftDestSize -= dictPosLimit;
        if (dictPosLimit == 0).as_bool() {
            break;
        }
        if (c_memcpy_s!(pDestOut, (pDecOut.destOutLen - leftDestSize).cast(), decCtx.dict + dictPos, dictPosLimit) != EOK!()).as_bool() {
            return CMPT_ERROR_MEM!();
        }
        pDestOut += dictPosLimit;
    } while (leftDestSize != 0).as_bool());

    pDecIn.strInCostLen = (pDecIn.strInLen - leftSrcSize).cast();
    pDecOut.destOutFillLen = (pDecOut.destOutLen - leftDestSize).cast();

    return res.cast();
}
