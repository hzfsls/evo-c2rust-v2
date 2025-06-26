pub fn CmptPriceLen(mut lenEncoder: Ptr<CmptLenEncoder>, mut len: u32, mut posState: u32) -> u32 {
    return lenEncoder.prices[posState][len - CMPTLZ_MATCH_LEN_MIN!()].cast();
}
