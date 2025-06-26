pub fn CmptRcDistProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut posSlot: u32, mut dist: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut footerBits: u32 = ((posSlot >> 1) - 1);
    if (dist < CMPT_DIST_LIMIT_2!()) {
        let mut base: u32 = ((2 | (posSlot & 1)) << footerBits);
        shiftRes = CmptRcReverseProcess(encCtx.rcCtx, (encCtx.probDistSpecial + base), footerBits, dist);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        let mut pos2: u32 = (dist | 0xF) << (32 - footerBits);
        let mut range: u32 = encCtx.rcCtx.range;
        c_do!({
            range >>= 1;
            encCtx.rcCtx.low += range & (0 - (pos2 >> 31));
            pos2 += pos2;
            CMPT_RC_NORMALIZE!(encCtx.rcCtx, range, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        } while pos2 != 0xF0000000);
        let mut m: u32 = 1;
        let mut bit: u32;
        let mut bit0Prob: u32;
        let mut newBound: u32;
        let mut k: i32;
        c_for!(k = 0; k < CMPTLZ_ALIGN_BITS!() - 1; k.suffix_plus_plus(); {
            bit = (dist & 1);
            dist >>= 1;
            CMPT_RC_BIT_PROCESS!(encCtx.rcCtx, encCtx.probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            m = (m << 1) + bit;
        });
        bit = (dist & 1);
        CMPT_RC_BIT_PROCESS!(encCtx.rcCtx, encCtx.probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        encCtx.rcCtx.range = range;
    }
    return CMPT_OK!();
}
