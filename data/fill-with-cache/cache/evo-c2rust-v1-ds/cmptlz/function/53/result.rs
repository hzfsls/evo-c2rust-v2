pub fn CmptEncShortOrRep0(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    if lenRes == 1 {
        shiftRes = CmptlzEncShortRep(encCtx.cast(), nowpos32.cast()).cast();
        if shiftRes != CMPT_OK!() {
            return shiftRes;
        }
    } else {
        shiftRes = CmptlzEncLongRep(encCtx.cast(), 0, nowpos32.cast(), lenRes.cast()).cast();
        if shiftRes != CMPT_OK!() {
            return shiftRes;
        }
    }
    return CMPT_OK!();
}
