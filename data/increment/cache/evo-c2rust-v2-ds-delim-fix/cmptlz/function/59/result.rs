pub fn CmptPriceSymbolReverse(mut encCtx: Ptr<CmptLzEncCtx>, mut symbolProbs: Ptr<CmptlzProb>, mut symbolBitsNum: u32, mut symbol: u32) -> u32 {
    let mut price: u32 = 0;
    let mut i: u32 = 1;
    c_do!({
        let mut bit: u32 = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx.cast(), symbolProbs[i].cast(), bit.cast()).cast::<u32>();
        i = (i << 1) + bit;
    } while symbolBitsNum.prefix_minus_minus());
    return price.cast();
}
