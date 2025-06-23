pub fn CmptPriceSymbol(mut encCtx: Ptr<CmptLzEncCtx>, mut symbolProbs: Ptr<CmptlzProb>, mut symbolBitsNum: u32, mut symbol: u32) -> u32 {
    let mut price: u32 = 0;
    symbol += 1 << symbolBitsNum;
    loop {
        let mut bit: u32 = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx.cast(), symbolProbs[symbol].cast(), bit.cast());
        if (symbol != 1).as_bool() {
            continue;
        } else {
            break;
        }
    }
    return price.cast();
}