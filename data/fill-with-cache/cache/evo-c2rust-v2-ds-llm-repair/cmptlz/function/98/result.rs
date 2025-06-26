pub fn CmptPriceSet(mut encCtx: Ptr<CmptLzEncCtx>, mut probs: Ptr<CmptlzProb>, mut startPrice: u32, mut prices: Ptr<u32>) {
    let mut i: u32 = 0;
    c_for!(i = 0; i < 8; i += 2; {
        let mut price: u32 = startPrice;
        let mut prob: u32;
        price += CmptPriceOneBit(encCtx, probs[1], (i >> 2));
        price += CmptPriceOneBit(encCtx, probs[2 + (i >> 2)], ((i >> 1) & 1));
        prob = probs[4 + (i >> 1)].cast();
        prices[i] = (price + CmptPriceBit0(encCtx, prob.cast()));
        prices[i + 1] = (price + CmptPriceBit1(encCtx, prob.cast()));
    });
}
