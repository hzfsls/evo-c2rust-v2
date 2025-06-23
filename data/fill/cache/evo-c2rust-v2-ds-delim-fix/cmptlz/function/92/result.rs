pub fn CmptPriceSymbol(mut encCtx: Ptr<CmptLzEncCtx>, mut symbolProbs: Ptr<CmptlzProb>, mut symbolBitsNum: u32, mut symbol: u32) -> u32 {
    let mut price: u32 = 0;
    symbol += (1 << symbolBitsNum).cast();
    c_do!({
        let mut bit: u32 = (symbol & 1).cast();
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx.cast(), symbolProbs[symbol].cast(), bit.cast()).cast();
    } while symbol != 1);
    return price.cast();
}
