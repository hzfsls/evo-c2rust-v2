pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32 = Default::default();
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut curMatch: u32 = Default::default();
    let mut lenLimit: u32 = mf.niceLen.cast();
    c_do!({
        lenLimit = mf.srcLen - mf.readPos;
        if (mf.niceLen <= lenLimit).as_bool() {
            lenLimit = mf.niceLen;
        } else {
            mf.readPos += 1;
            continue;
        }
        let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
        pos = mf.readPos + mf.offset;
        CMPT_HASH_4_CALC!(mf.cast(), cur.cast(), temp.cast(), hash2Value.cast(), hash3Value.cast(), hashValue.cast());
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH + hashValue].cast();
        CMPT_HASH_UPDATE!(mf.cast(), hash2Value.cast(), hash3Value.cast(), hashValue.cast(), pos.cast());
        CmptBtSkip(mf.cast(), lenLimit.cast(), pos.cast(), cur.cast(), curMatch.cast());
        CMPT_MF_MOVE_POS!(mf.cast());
    } while amount.suffix_minus_minus() != 0);
}