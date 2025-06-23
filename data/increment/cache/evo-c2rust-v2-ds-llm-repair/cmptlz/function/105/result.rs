pub fn CmptLzDecDecodeToBuf(mut decCtx: Ptr<CmptLzDecCtx>, mut pDecIn: Ptr<CmptLzDecIn>, mut pDecOut: Ptr<CmptLzDecOut>, mut finMode: EnCmptLzFinMode, mut finStatus: Ptr<EnCmptLzStatus>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut dictPos: usize;
    let mut dictPosLimit: usize;
    let mut srcCostSize: usize;

    let mut leftSrcSize: usize = pDecIn.strInLen;
    let mut leftDestSize: usize = pDecOut.destOutLen;
    let mut pSrcIn: Ptr<u8> = pDecIn.pSrcIn;
    let mut pDestOut: Ptr<u8> = pDecOut.pDestOut;
    let mut tmpFinMode: EnCmptLzFinMode;

    c_do!({
        if (decCtx.dictPos == decCtx.dictBufSize) {
            decCtx.dictPos = 0;
        }
        dictPos = decCtx.dictPos;
        if (leftDestSize > decCtx.dictBufSize - dictPos) {
            dictPosLimit = decCtx.dictBufSize;
            tmpFinMode = CMPTLZ_FINISH_ANY!();
        } else {
            dictPosLimit = (dictPos + leftDestSize);
            tmpFinMode = finMode;
        }
        srcCostSize = leftSrcSize;
        res = CmptLzDecDecodeToDic(decCtx, dictPosLimit, pSrcIn, c_ref!(srcCostSize), tmpFinMode, finStatus);
        if (res != CMPT_OK!()) {
            break;
        }
        pSrcIn += srcCostSize;
        leftSrcSize -= srcCostSize;
        dictPosLimit = (decCtx.dictPos - dictPos);
        leftDestSize -= dictPosLimit;
        if (dictPosLimit == 0) {
            break;
        }
        if (c_memcpy_s!(pDestOut, (pDecOut.destOutLen - leftDestSize), decCtx.dict + dictPos, dictPosLimit) != EOK!()) {
            return CMPT_ERROR_MEM!();
        }
        pDestOut += dictPosLimit;
    } while (leftDestSize != 0));

    pDecIn.strInCostLen = (pDecIn.strInLen - leftSrcSize);
    pDecOut.destOutFillLen = (pDecOut.destOutLen - leftDestSize);

    return res;
}
