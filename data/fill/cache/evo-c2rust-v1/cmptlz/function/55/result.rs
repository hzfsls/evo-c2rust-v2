pub fn CmptEncodeAll(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx.cast();
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx.cast();
    if mf.srcLen == 0 {
        return CmptlzFlush(encCtx.cast()).cast();
    }
    if encCtx.nowpos64 == 0 {
        let mut range: u32;
        let mut bit0Prob: u32;
        let mut newBound: u32;
        range = rc.range.cast();
        let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[encCtx.state][0]).cast();
        CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
        let mut shiftRes: i32 = CMPT_OK!();
        CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        rc.range = range.cast();
        let mut curByte: u8 = (*mf.srcStart).cast();
        let mut litProb: Ptr<CmptlzProb> = c_ref!(encCtx.litMarcov.literal[0][0]).cast();
        shiftRes = CmptRcLitProcess(rc.cast(), litProb.cast(), curByte.cast()).cast();
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        mf.mfStart += 1;
        encCtx.nowpos64 += 1;
        mf.readPos += 1;
        if mf.srcLen == 1 {
            return CmptlzFlush(encCtx.cast()).cast();
        }
    }
    let mut res: i32;
    loop {
        res = CmptEncodeOneBlock(encCtx.cast()).cast();
        if res != 0 || encCtx.encNeedFinish {
            break;
        }
    }
    return res.cast();
}
