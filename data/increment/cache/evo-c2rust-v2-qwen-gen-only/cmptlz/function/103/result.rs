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
        if CMPTLZ_PRINTF_ENC_PROCESS!().as_bool() {
            c_printf!(
                cstr!(" now in CmptEncodeOneBlock process, backRes is %u, lenRes is %u\n"),
                backRes.cast(),
                lenRes.cast()
            );
            c_printf!(cstr!(" nowpos32 is %u\n"), nowpos32.cast());
        }
        if backRes == CMPTLZ_UINT32_MAX!() {
            shiftRes = CmptlzEncLit(encCtx.cast(), mf.cast(), nowpos32.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        } else if backRes == 0 {
            shiftRes = CmptEncShortOrRep0(encCtx.cast(), nowpos32.cast(), lenRes.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        } else if backRes == 1 {
            shiftRes = CmptlzEncLongRep(encCtx.cast(), 1.cast(), nowpos32.cast(), lenRes.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        } else if backRes == 2 {
            shiftRes = CmptlzEncLongRep(encCtx.cast(), 2.cast(), nowpos32.cast(), lenRes.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        } else if backRes == 3 {
            shiftRes = CmptlzEncLongRep(encCtx.cast(), 3.cast(), nowpos32.cast(), lenRes.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        } else {
            shiftRes = CmptlzEncNormalMatch(encCtx.cast(), nowpos32.cast(), backRes.cast(), lenRes.cast()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes.cast();
            }
        }
        nowpos32 += lenRes;
        mf.mfStart += lenRes;
        mf.readAhead -= lenRes;
        if mf.readAhead == 0 {
            CmptPriceCheck(encCtx.cast());
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
    return CmptlzFlush(encCtx.cast()).cast();
}