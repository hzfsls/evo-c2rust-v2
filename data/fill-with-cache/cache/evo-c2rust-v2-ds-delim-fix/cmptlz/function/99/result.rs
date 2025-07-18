pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0];
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx.cast(), prob.cast()).cast();
    a = CmptPriceBit0(encCtx.cast(), prob.cast()).cast();
    c = (b + CmptPriceBit0(encCtx.cast(), lenEncoder.low[1 << CMPT_LEN_LOW_BITS!()].cast()).cast()).cast();
    c_for!(posState = 0; posState < numPosStates; posState.suffix_plus_plus(); {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState].cast();
        let mut probs: Ptr<CmptlzProb> = (lenEncoder.low + (posState << (1 + CMPT_LEN_LOW_BITS!()))).cast();
        CmptPriceSet(encCtx.cast(), probs.cast(), a.cast(), prices.cast());
        CmptPriceSet(encCtx.cast(), (probs + (1 << CMPT_LEN_LOW_BITS!())).cast(), c.cast(), (prices + (1 << CMPT_LEN_LOW_BITS!())).cast());
    });
    let mut i: u32 = lenEncoder.tableSize.cast();
    if (i > (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()).as_bool() {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high.cast();
        let mut prices: Ptr<u32> = (lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()).cast();
        i -= (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx.cast(), lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!())].cast()).cast();
        c_do!({
            let mut sym: u32 = (i.prefix_minus_minus() + (1 << (CMPT_LEN_HIGH_BITS!() - 1))).cast();
            let mut price: u32 = b.cast();
            c_do!({
                let mut bit: u32 = (sym & 1).cast();
                sym >>= 1;
                price += CmptPriceOneBit(encCtx.cast(), probs[sym].cast(), bit.cast()).cast();
            } while sym >= 2);
            prob = probs[(i + (1 << (CMPT_LEN_HIGH_BITS!() - 1)))].cast();
            prices[(i * CMPT_DOUBLE!())] = (price + CmptPriceBit0(encCtx.cast(), prob.cast())).cast();
            prices[(i * CMPT_DOUBLE!() + 1)] = (price + CmptPriceBit1(encCtx.cast(), prob.cast())).cast();
        } while i != 0);
        let mut num: usize = (lenEncoder.tableSize - (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) * c_sizeofval!(lenEncoder.prices[0][0]);
        c_for!(posState = 1; posState < numPosStates; posState.suffix_plus_plus(); {
            c_memcpy_s!((lenEncoder.prices[posState] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()).cast(), CMPT_MF_LONGEST_MATCH!() - 1, (lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()).cast(), num);
        });
    }
}
