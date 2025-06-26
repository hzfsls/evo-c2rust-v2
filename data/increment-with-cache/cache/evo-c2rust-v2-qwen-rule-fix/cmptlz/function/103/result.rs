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
        if CMPTLZ_PRINTF_ENC_PROCESS!() {
            c_printf!(
                cstr!(" now in CmptEncodeOneBlock process, backRes is %u, lenRes is %u\n"),
                backRes,
                lenRes
            );
            c_printf!(cstr!(" nowpos32 is %u\n"), nowpos32);
        }
        if backRes == CMPTLZ_UINT32_MAX!() {
            shiftRes = CmptlzEncLit(encCtx, mf, nowpos32);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        } else if backRes == 0 {
            shiftRes = CmptEncShortOrRep0(encCtx, nowpos32, lenRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        } else if backRes == 1 {
            shiftRes = CmptlzEncLongRep(encCtx, 1, nowpos32, lenRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        } else if backRes == 2 {
            shiftRes = CmptlzEncLongRep(encCtx, 2, nowpos32, lenRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        } else if backRes == 3 {
            shiftRes = CmptlzEncLongRep(encCtx, 3, nowpos32, lenRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        } else {
            shiftRes = CmptlzEncNormalMatch(encCtx, nowpos32, backRes, lenRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
        }
        nowpos32 += lenRes;
        mf.mfStart += lenRes;
        mf.readAhead -= lenRes;
        if mf.readAhead == 0 {
            CmptPriceCheck(encCtx);
            if mf.srcLen <= mf.mfStart {
                break;
            }
            if (nowpos32 - startpos) >= CMPT_ONE_BLOCK_MAX_SIZE!() {
                encCtx.nowpos64 += nowpos32 - startpos;
                return 0;
            }
        }
    }
    encCtx.nowpos64 += nowpos32 - startpos;
    return CmptlzFlush(encCtx);
}