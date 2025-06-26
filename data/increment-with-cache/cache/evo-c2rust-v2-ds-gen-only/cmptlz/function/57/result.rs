pub fn CmptPriceBit1(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb) -> u32 {
    return (*encCtx.priceRootTable)[(bit0Prob ^ (CMPTLZ_PROB_MAX_NUM!() - 1)) >> CMPT_PRICE_BITS_MOVING_NUM!()].cast();
}
