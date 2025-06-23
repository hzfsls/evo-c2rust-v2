pub fn CmptEncodeOneBlock(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx;
    let mut nowpos32: u32 = encCtx.nowpos64.cast();
    let mut startpos: u32 = nowpos32;
    let mut backRes: u32 = Default::default();
    let mut lenRes: u32 = Default::default();
    let mut shiftRes: i32 = CMPT_OK!();
    loop {
        CmptlzDp(encCtx, mf, nowpos32);
        backRes = encCtx.backRes;
        lenRes = encCtx.lenRes;
        c_switch!(backRes; {
            CMPTLZ_UINT32_MAX!() => {
                shiftRes = CmptlzEncLit(encCtx, mf, nowpos32);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
            0 => {
                shiftRes = CmptEncShortOrRep0(encCtx, nowpos32, lenRes);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
            1 => {
                shiftRes = CmptlzEncLongRep(encCtx, 1, nowpos32, lenRes);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
            2 => {
                shiftRes = CmptlzEncLongRep(encCtx, 2, nowpos32, lenRes);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
            3 => {
                shiftRes = CmptlzEncLongRep(encCtx, 3, nowpos32, lenRes);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
            _ => {
                shiftRes = CmptlzEncNormalMatch(encCtx, nowpos32, backRes, lenRes);
                if shiftRes != CMPT_OK!() {
                    return shiftRes;
                }
                break;
            },
        });
        nowpos32 += lenRes;
        mf.mfStart += lenRes;
        mf.readAhead -= lenRes;
        if (mf.readAhead == 0) {
            CmptPriceCheck(encCtx);
            if (mf.srcLen <= mf.mfStart.try_into().unwrap()) {
                break;
            }
            if (nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE!()) {
                encCtx.nowpos64 += (nowpos32 - startpos).cast();
                return 0;
            }
        }
    }
    encCtx.nowpos64 += (nowpos32 - startpos).cast();
    return CmptlzFlush(encCtx);
}
