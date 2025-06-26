pub fn CmptEncodeOneBlock(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx.cast();
    let mut nowpos32: u32 = encCtx.nowpos64.cast();
    let mut startpos: u32 = nowpos32.cast();
    let mut backRes: u32 = Default::default();
    let mut lenRes: u32 = Default::default();
    let mut shiftRes: i32 = CMPT_OK!();
    loop {
        CmptlzDp(encCtx.cast(), mf.cast(), nowpos32.cast());
        backRes = encCtx.backRes.cast();
        lenRes = encCtx.lenRes.cast();
        c_switch!(backRes, {
            CMPTLZ_UINT32_MAX!() => {
                shiftRes = CmptlzEncLit(encCtx.cast(), mf.cast(), nowpos32.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            0 => {
                shiftRes = CmptEncShortOrRep0(encCtx.cast(), nowpos32.cast(), lenRes.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            1 => {
                shiftRes = CmptlzEncLongRep(encCtx.cast(), 1, nowpos32.cast(), lenRes.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            2 => {
                shiftRes = CmptlzEncLongRep(encCtx.cast(), 2, nowpos32.cast(), lenRes.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            3 => {
                shiftRes = CmptlzEncLongRep(encCtx.cast(), 3, nowpos32.cast(), lenRes.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            _ => {
                shiftRes = CmptlzEncNormalMatch(encCtx.cast(), nowpos32.cast(), backRes.cast(), lenRes.cast()).cast();
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
        });
        nowpos32 += lenRes;
        mf.mfStart += lenRes;
        mf.readAhead -= lenRes;
        if (mf.readAhead == 0).as_bool() {
            CmptPriceCheck(encCtx.cast());
            if (mf.srcLen <= mf.mfStart).as_bool() {
                break;
            }
            if (nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE!()).as_bool() {
                encCtx.nowpos64 += (nowpos32 - startpos).cast();
                return 0;
            }
        }
    }
    encCtx.nowpos64 += (nowpos32 - startpos).cast();
    return CmptlzFlush(encCtx.cast()).cast();
}
