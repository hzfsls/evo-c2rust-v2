pub fn CmptPriceSymbolReverse(mut encCtx: Ptr<CmptLzEncCtx>, mut symbolProbs: Ptr<CmptlzProb>, mut symbolBitsNum: u32, mut symbol: u32) -> u32 {
    let mut price: u32 = 0;
    let mut i: u32 = 1;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[i], bit);
        i = (i << 1) + bit;
        symbolBitsNum -= 1;
        if symbolBitsNum == 0 {
            break;
        }        
    }
    return price;
}