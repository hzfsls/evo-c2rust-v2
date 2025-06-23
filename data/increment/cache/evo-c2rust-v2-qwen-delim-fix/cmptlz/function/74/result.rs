pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let mut numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0];
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx.cast(), prob.cast()).cast();
    a = CmptPriceBit0(encCtx.cast(), prob.cast()).cast();
    c = b + CmptPriceBit0(encCtx.cast(), lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!()).cast()].cast()).cast();
    c_for!(posState = 0; posState < numPosStates; posState.suffix_plus_plus(); {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState];
        let mut probs: Ptr<CmptlzProb> = lenEncoder.low + (posState << (1 + CMPT_LEN_LOW_BITS!())).cast();
        CmptPriceSet(encCtx.cast(), probs.cast(), a.cast(), prices.cast());
        CmptPriceSet(encCtx.cast(), probs.cast() + (1 << CMPT_LEN_LOW_BITS!()).cast(), c.cast(), prices.cast() + (1 << CMPT_LEN_LOW_BITS!()).cast());
    });
    let mut i: u32 = lenEncoder.tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!()).as_bool() {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high;
        let mut prices: Ptr<u32> = lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!();
        i -= (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx.cast(), lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!()).cast()].cast()).cast();
        c_do!({
            let mut sym: u32 = (i.suffix_minus_minus() + (1 << (CMPT_LEN_HIGH_BITS!() - 1))).cast();
            let mut price: u32 = b;
            c_do!({
                let mut bit: u32 = sym & 1;
                sym >>= 1;
                price += CmptPriceOneBit(encCtx.cast(), probs[sym].cast(), bit.cast());
            } while sym >= 2);
            let mut prob: u32 = probs[(i.cast::<usize>()) + (1 << (CMPT_LEN_HIGH_BITS!() - 1))].cast();
            prices[(i.cast::<usize>()) * CMPT_DOUBLE!()] = price + CmptPriceBit0(encCtx.cast(), prob.cast()).cast();
            prices[(i.cast::<usize>()) * CMPT_DOUBLE!() + 1] = price + CmptPriceBit1(encCtx.cast(), prob.cast()).cast();
        } while i != 0);
        let mut num: usize = (lenEncoder.tableSize - (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!()) * c_sizeofval!(lenEncoder.prices[0][0]).cast();
        c_for!(posState = 1; posState < numPosStates; posState.suffix_plus_plus(); {
            c_memcpy_s!(lenEncoder.prices[posState].cast() + (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!(), CMPT_MF_LONGEST_MATCH!() - 1, lenEncoder.prices[0].cast() + (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!(), num.cast());
        });
    }
}