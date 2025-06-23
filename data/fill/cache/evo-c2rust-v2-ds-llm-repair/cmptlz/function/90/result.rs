pub fn CmptPriceBit0(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb) -> u32 {
    return encCtx.priceRootTable[(bit0Prob >> CMPT_PRICE_BITS_MOVING_NUM!()).cast::<usize>()].cast();
}
