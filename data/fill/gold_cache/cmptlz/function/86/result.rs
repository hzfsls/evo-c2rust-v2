pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32;
    let mut temp: u32;
    let mut hash2Value: u32;
    let mut hash3Value: u32;
    let mut hashValue: u32;
    let mut curMatch: u32;
    let niceLen: u32 = mf.niceLen;
    loop {
        let mut lenLimit: u32 = mf.srcLen as u32 - mf.readPos;
        if CMPTLZ_LIKELY!(niceLen <= lenLimit) {
            lenLimit = niceLen;
        } else {
            mf.readPos += 1;
            // do while continue here
            amount -= 1;
            if amount == 0 {
                break;
            }
            continue;
        }
        let cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast();
        pos = mf.readPos + mf.offset;
        CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue];
        CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf, lenLimit, pos, cur, curMatch);
        CMPT_MF_MOVE_POS!(mf);
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}