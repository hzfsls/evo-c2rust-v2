pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32 = Default::default();
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut curMatch: u32 = Default::default();
    let mut niceLen: u32 = mf.niceLen.cast();
    c_do!({
        let mut lenLimit: u32 = (mf.srcLen - mf.readPos).cast();
        if CMPTLZ_LIKELY!(niceLen <= lenLimit) {
            lenLimit = niceLen.cast();
        } else {
            mf.readPos += 1;
            continue;
        }
        let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast();
        pos = (mf.readPos + mf.offset).cast();
        CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue].cast();
        CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf.cast(), lenLimit.cast(), pos.cast(), cur.cast(), curMatch.cast());
        CMPT_MF_MOVE_POS!(mf);
    } while amount.prefix_minus_minus() != 0);
}
