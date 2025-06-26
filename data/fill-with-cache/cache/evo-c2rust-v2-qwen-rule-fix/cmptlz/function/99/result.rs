pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let mut numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0];
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx, prob.cast());
    a = CmptPriceBit0(encCtx, prob.cast());
    c = b + CmptPriceBit0(encCtx, lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!())]);
    c_for!(posState: u32 = 0; posState < numPosStates; posState.suffix_plus_plus(); {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState];
        let mut probs: Ptr<CmptlzProb> = lenEncoder.low + (posState << (1 + CMPT_LEN_LOW_BITS!()));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, probs + (1 << CMPT_LEN_LOW_BITS!()), c, prices + (1 << CMPT_LEN_LOW_BITS!()));
    });
    let mut i: u32 = lenEncoder.tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high;
        let mut prices: Ptr<u32> = lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!();
        i -= (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!())]);
        c_do!({
            let mut sym: u32 = i.suffix_minus_minus() + (1 << (CMPT_LEN_HIGH_BITS!() - 1));
            let mut price: u32 = b;
            c_do!({
                let mut bit: u32 = sym & 1;
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym], bit);
            } while sym >= 2);
            let tmp0 = (i.cast::<usize>()) + (1 << (CMPT_LEN_HIGH_BITS!() - 1));
            let mut prob: u32 = probs[tmp0].cast();
            prices[(i.cast::<usize>()) * CMPT_DOUBLE!()] = price + CmptPriceBit0(encCtx, prob.cast());
            prices[(i.cast::<usize>()) * CMPT_DOUBLE!() + 1] = price + CmptPriceBit1(encCtx, prob.cast());
        } while i);
        let mut num: usize = (lenEncoder.tableSize - (1 << CMPT_LEN_LOW_BITS!()).cast() * CMPT_DOUBLE!()) * c_sizeofval!(lenEncoder.prices[0][0]).cast::<usize>();
        c_for!(posState: u32 = 1; posState < numPosStates; posState.suffix_plus_plus(); {
            c_memcpy_s!(lenEncoder.prices[posState] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), CMPT_MF_LONGEST_MATCH!() - 1, lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), num);
        });
    }
}