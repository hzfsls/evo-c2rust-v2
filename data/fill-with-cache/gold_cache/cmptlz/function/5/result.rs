pub fn CmptLzPropsDecode(mut protData: Ptr<u8>, mut protSize: u32, mut decProt: Ptr<CmptLzDecProt>) -> i32 {
    let mut dictSize: u32;
    if protSize < CMPTLZ_PROPS_SIZE!() {
        return CMPT_ERROR_UNSUPPORTED!();
    } else {
        dictSize = protData[1] as u32 | ((protData[2] as u32) << 8) | ((protData[3] as u32) << 16) | ((protData[4] as u32) << 24);
    }
    if dictSize < CMPTLZ_DICT_MIN_LEN!() {
        dictSize = CMPTLZ_DICT_MIN_LEN!();
    }
    decProt.dicSize = dictSize;
    let mut firstData: u8 = protData[0];
    if firstData >= (CMPTLZ_LIT_CTX_MAX!() * CMPTLZ_POS_STATE_MAX!() * CMPTLZ_LIT_POS_MAX!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    }
    decProt.litCtx = (firstData % CMPTLZ_LIT_CTX_MAX!()) as u8;
    firstData /= CMPTLZ_LIT_CTX_MAX!();
    decProt.posBits = (firstData / CMPTLZ_POS_STATE_MAX!()) as u8;
    decProt.litPos = (firstData % CMPTLZ_LIT_POS_MAX!()) as u8;
    return CMPT_OK!();
}