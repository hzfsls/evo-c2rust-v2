pub fn CmptMfMovePos(mut mf: Ptr<CmptMfCtx>) {
    let subvalue: u32 = CMPTLZ_UINT32_MAX!() - mf.cycleSize;
    let mut i: u32;
    c_for!(i = 0; i < mf.hashCount; i += 1; {
        if mf.hash[i] <= subvalue {
            mf.hash[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.hash[i] -= subvalue;
        }
    });
    c_for!(i = 0; i < mf.sonsCount; i += 1; {
        if mf.son[i] <= subvalue {
            mf.son[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.son[i] -= subvalue;
        }
    });
    mf.offset -= subvalue;
}