pub fn CmptEncShortOrRep0(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32;
    if (lenRes == 1).as_bool() {
        shiftRes = CmptlzEncShortRep(encCtx.cast(), nowpos32.cast()).cast();
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes.cast();
        }
    } else {
        shiftRes = CmptlzEncLongRep(encCtx.cast(), 0.cast(), nowpos32.cast(), lenRes.cast()).cast();
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes.cast();
        }
    }
    return CMPT_OK!();
}