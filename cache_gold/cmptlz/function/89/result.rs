pub fn CmptPriceOneBit(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb, mut curbit: u32) -> u32 {
    return encCtx.priceRootTable[(bit0Prob as u32 ^ ((0 - curbit) as u32 & (CMPTLZ_PROB_MAX_NUM!() - 1))) >> CMPT_PRICE_BITS_MOVING_NUM!()];
}