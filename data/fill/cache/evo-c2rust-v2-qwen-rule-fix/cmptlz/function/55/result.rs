pub fn CmptEncodeAll(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx;
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx;
    if (mf.srcLen == 0) {
        return CmptlzFlush(encCtx);
    }
    if (encCtx.nowpos64 == 0) {
        let mut range: u32 = rc.range;
        let tmp0 = encCtx.state;
        let mut probs: Ptr<CmptlzProb> = &encCtx.isMatch[tmp0][0];
        let mut bit0Prob: u32 = Default::default();
        let mut newBound: u32 = Default::default();
        CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
        let mut shiftRes: i32 = CMPT_OK!();
        CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        rc.range = range;
        let mut curByte: u8 = (*mf.srcStart);
        let mut litProb: Ptr<CmptlzProb> = &encCtx.litMarcov.literal[0][0];
        shiftRes = CmptRcLitProcess(rc, litProb, curByte.cast());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        mf.mfStart += 1;
        encCtx.nowpos64 += 1;
        mf.readPos += 1;
        if (mf.srcLen == 1) {
            return CmptlzFlush(encCtx);
        }
    }
    let mut res: i32 = Default::default();
    loop {
        res = CmptEncodeOneBlock(encCtx);
        if (res != 0) || encCtx.encNeedFinish {
            break;
        }
    }
    return res;
}