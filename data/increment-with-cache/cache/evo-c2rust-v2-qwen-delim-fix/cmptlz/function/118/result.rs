pub fn CmptRcPosSlotProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut posSlot: u32, mut len: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = encCtx.rcCtx.range.cast();
    let mut sym: u32 = posSlot + (1 << 6);
    let mut bit0Prob: u32 = Default::default();
    let mut newBound: u32 = Default::default();
    let mut bit: u32 = Default::default();
    let mut probs: Ptr<CmptlzProb> = encCtx.probDistSlot[GET_LEN_TO_POS_STATE!(len)].cast();
    loop {
        let mut posSlotProbTableIndex: Ptr<CmptlzProb> = probs + (sym >> CMPTLZ_DIST_SLOT_BITS!()).cast();
        bit = (sym >> (CMPTLZ_DIST_SLOT_BITS!() - 1)) & 1;
        sym <<= 1;
        CMPT_RC_BIT_PROCESS!(encCtx.rcCtx.cast(), posSlotProbTableIndex.cast(), bit.cast(), bit0Prob.cast(), range.cast(), newBound.cast(), shiftRes.cast());
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes.cast();
        }
        if (sym < (1 << (CMPTLZ_DIST_SLOT_BITS!() * 2))).as_bool() {
            continue;
        } else {
            break;
        }
    }
    encCtx.rcCtx.range = range.cast();
    return CMPT_OK!();
}