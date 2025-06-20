pub fn CmptRcPosSlotProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut posSlot: u32, mut len: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = encCtx.rcCtx.range;
    let mut sym: u32 = posSlot + (1 << 6);
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut bit: u32;
    let mut probs: Ptr<CmptlzProb> = encCtx.probDistSlot[GET_LEN_TO_POS_STATE!(len)].cast();
    loop {
        let mut posSlotProbTableIndex: Ptr<CmptlzProb> = probs + (sym >> CMPTLZ_DIST_SLOT_BITS!());
        bit = (sym >> (CMPTLZ_DIST_SLOT_BITS!() - 1)) & 1;
        sym <<= 1;
        CMPT_RC_BIT_PROCESS!(encCtx.rcCtx, posSlotProbTableIndex, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        if sym >= (1 << (CMPTLZ_DIST_SLOT_BITS!() * 2)) {
            break;
        }
    }
    encCtx.rcCtx.range = range;
    return CMPT_OK!();
}