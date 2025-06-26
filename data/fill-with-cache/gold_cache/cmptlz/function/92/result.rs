pub fn CmptPriceSymbol(mut encCtx: Ptr<CmptLzEncCtx>, mut symbolProbs: Ptr<CmptlzProb>, mut symbolBitsNum: u32, mut symbol: u32) -> u32 {
    let mut price: u32 = 0;
    symbol += 1 << symbolBitsNum;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[symbol], bit);
        if symbol == 1 {
            break;
        }
    }
    return price;
}