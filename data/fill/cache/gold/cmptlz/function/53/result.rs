pub fn CmptEncShortOrRep0(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    if lenRes == 1 {
        shiftRes = CmptlzEncShortRep(encCtx, nowpos32);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        shiftRes = CmptlzEncLongRep(encCtx, 0, nowpos32, lenRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}