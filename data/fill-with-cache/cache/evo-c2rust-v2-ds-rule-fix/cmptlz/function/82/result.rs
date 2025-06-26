pub fn CmptMfMovePos(mut mf: Ptr<CmptMfCtx>) {
    let mut subvalue: u32 = (CMPTLZ_UINT32_MAX!() - mf.cycleSize).cast();
    let mut i: u32 = Default::default();
    c_for!(i = 0; i < mf.hashCount; i.suffix_plus_plus(); {
        if (mf.hash[i] <= subvalue).as_bool() {
            mf.hash[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.hash[i] -= subvalue;
        }
    });
    c_for!(i = 0; i < mf.sonsCount; i.prefix_plus_plus(); {
        if (mf.son[i] <= subvalue).as_bool() {
            mf.son[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.son[i] -= subvalue;
        }
    });
    mf.offset -= subvalue;
}
