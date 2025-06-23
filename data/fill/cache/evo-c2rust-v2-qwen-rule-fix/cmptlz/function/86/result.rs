pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32 = Default::default();
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut curMatch: u32 = Default::default();
    let mut lenLimit: u32 = mf.niceLen;
    c_do!({
        lenLimit = mf.srcLen - mf.readPos;
        if (mf.niceLen <= lenLimit) {
            lenLimit = mf.niceLen;
        } else {
            mf.readPos += 1;
            continue;
        }
        let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
        pos = mf.readPos + mf.offset;
        CMPT_HASH_4_CALC!(mf, cur.cast(), temp, hash2Value, hash3Value, hashValue);
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH + hashValue];
        CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf, lenLimit, pos, cur, curMatch);
        CMPT_MF_MOVE_POS!(mf);
    } while amount.suffix_minus_minus() != 0);
}