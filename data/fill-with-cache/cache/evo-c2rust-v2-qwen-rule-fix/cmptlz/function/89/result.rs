pub fn CmptPriceOneBit(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb, mut curbit: u32) -> u32 {
    let mut index: u32 = (bit0Prob ^ ((0 - curbit) & (CMPTLZ_PROB_MAX_NUM!() - 1))).cast();
    return encCtx.priceRootTable[index >> CMPT_PRICE_BITS_MOVING_NUM!()];
}