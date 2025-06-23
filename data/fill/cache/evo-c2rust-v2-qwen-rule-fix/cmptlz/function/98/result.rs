pub fn CmptPriceSet(mut encCtx: Ptr<CmptLzEncCtx>, mut probs: Ptr<CmptlzProb>, mut startPrice: u32, mut prices: Ptr<u32>) {
    let mut i: u32;
    c_for!(let mut i: u32 = 0; i < 8; i += 2; {
        let mut price: u32 = startPrice;
        let mut prob: CmptlzProb = Default::default();
        price += CmptPriceOneBit(encCtx.cast(), probs[1].cast(), (i >> 2).cast());
        price += CmptPriceOneBit(encCtx.cast(), probs[2 + (i >> 2)].cast(), ((i >> 1) & 1).cast());
        prob = probs[4 + (i >> 1)].cast();
        prices[i] = (price + CmptPriceBit0(encCtx.cast(), prob.cast())).cast();
        prices[i + 1] = (price + CmptPriceBit1(encCtx.cast(), prob.cast())).cast();
    });
}